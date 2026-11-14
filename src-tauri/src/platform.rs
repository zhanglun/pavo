//! 系统壁纸设置平台层。
//!
//! 该层是"应用到系统"的唯一出口：Windows 走 IDesktopWallpaper（Win8+ 逐屏接口），
//! macOS 走 NSWorkspace 逐屏 API，Linux 保留 wallpaper crate 的 DE 命令实现。
//! 语义对齐旧实现：所有屏幕设为同一张图 + 填充（Fill）模式。

use std::sync::OnceLock;

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

pub type PlatformResult<T> = Result<T, String>;

/// 注入应用句柄（main.rs setup 阶段调用一次）。
/// macOS 的 AppKit 调用必须在主线程执行，需要借它 run_on_main_thread。
pub fn set_app_handle(handle: tauri::AppHandle) -> Result<(), tauri::AppHandle> {
  APP_HANDLE.set(handle)
}

/// 取注入的应用句柄。仅 macOS 分支需要。
#[cfg(target_os = "macos")]
fn app_handle() -> PlatformResult<tauri::AppHandle> {
  APP_HANDLE
    .get()
    .cloned()
    .ok_or_else(|| "[mac] app handle not initialized".to_string())
}

/// 所有屏幕设为同一张壁纸（填充模式）。
pub fn set_wallpaper_all(path: &str) -> PlatformResult<()> {
  #[cfg(target_os = "windows")]
  return windows_impl::set_wallpaper_all(path);
  #[cfg(target_os = "macos")]
  return macos_impl::set_wallpaper_all(path);
  #[cfg(target_os = "linux")]
  return linux_impl::set_wallpaper_all(path);
}

#[cfg(target_os = "windows")]
mod windows_impl {
  use windows::core::{HSTRING, PCWSTR};
  use windows::Win32::Foundation::RPC_E_CHANGED_MODE;
  use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
  };
  use windows::Win32::UI::Shell::{DesktopWallpaper, DWPOS_FILL, IDesktopWallpaper};

  pub fn set_wallpaper_all(path: &str) -> super::PlatformResult<()> {
    unsafe {
      // 每次调用线程独立初始化 COM；RPC_E_CHANGED_MODE 表示线程已是 MTA，直接复用且不需 Uninit
      let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
      if hr.is_err() && hr != RPC_E_CHANGED_MODE {
        return Err(format!("[win] CoInitializeEx failed: {hr}"));
      }
      let need_uninit = hr.is_ok();

      let result = set_all_inner(path);

      if need_uninit {
        CoUninitialize();
      }
      result
    }
  }

  /// monitorID 传 NULL 时 SetWallpaper 作用于所有显示器；SetPosition 为全局填充语义。
  unsafe fn set_all_inner(path: &str) -> super::PlatformResult<()> {
    let wallpaper: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)
      .map_err(|e| format!("[win] create DesktopWallpaper failed: {e}"))?;

    wallpaper
      .SetWallpaper(PCWSTR::null(), &HSTRING::from(path))
      .map_err(|e| format!("[win] SetWallpaper failed: {e}"))?;

    wallpaper
      .SetPosition(DWPOS_FILL)
      .map_err(|e| format!("[win] SetPosition failed: {e}"))?;

    Ok(())
  }
}

#[cfg(target_os = "macos")]
mod macos_impl {
  use objc2::MainThreadMarker;
  use objc2::rc::{autoreleasepool, Retained};
  use objc2::runtime::AnyObject;
  use objc2_app_kit::{
    NSImageScaling, NSWorkspace, NSWorkspaceDesktopImageOptionKey, NSWorkspaceDesktopImageScalingKey,
    NSScreen,
  };
  use objc2_foundation::{NSDictionary, NSNumber, NSURL};
  use std::path::Path;
  use std::sync::mpsc;
  use std::time::Duration;

  pub fn set_wallpaper_all(path: &str) -> super::PlatformResult<()> {
    let handle = super::app_handle()?;
    let path = path.to_string();
    let (tx, rx) = mpsc::channel::<super::PlatformResult<()>>();

    handle
      .run_on_main_thread(move || {
        let result = autoreleasepool(|| unsafe { set_all_on_main(&path) });
        let _ = tx.send(result);
      })
      .map_err(|e| format!("[mac] dispatch to main thread failed: {e}"))?;

    // 主线程被阻塞时兜底超时，避免调用线程永久挂起
    match rx.recv_timeout(Duration::from_secs(10)) {
      Ok(result) => result,
      Err(_) => Err("[mac] wallpaper task did not complete in time".to_string()),
    }
  }

  unsafe fn set_all_on_main(path: &str) -> super::PlatformResult<()> {
    let mtm = MainThreadMarker::new()
      .ok_or_else(|| "[mac] not running on main thread".to_string())?;
    let screens = NSScreen::screens(mtm);
    let workspace = NSWorkspace::sharedWorkspace();
    let options = build_fill_options();
    let url = NSURL::from_file_path(Path::new(path))
      .ok_or_else(|| format!("[mac] invalid wallpaper path: {path}"))?;

    // 遍历所有屏幕逐屏设置，与旧 osascript "tell every desktop" 行为一致
    for screen in screens.to_vec().iter() {
      workspace
        .setDesktopImageURL_forScreen_options_error(&url, screen, &options)
        .map_err(|e| format!("[mac] set desktop image failed: {e:?}"))?;
    }
    Ok(())
  }

  /// 构造填充缩放选项：保持与旧实现的视觉语义一致（按比例放大/缩小填满屏幕）。
  unsafe fn build_fill_options() -> Retained<NSDictionary<NSWorkspaceDesktopImageOptionKey, AnyObject>> {
    let scaling = NSNumber::new_isize(NSImageScaling::NSImageScaleProportionallyUpOrDown.0);
    let any: Retained<AnyObject> = scaling.into_super();
    NSDictionary::from_retained_objects(&[NSWorkspaceDesktopImageScalingKey], &[any])
  }
}

#[cfg(target_os = "linux")]
mod linux_impl {
  pub fn set_wallpaper_all(path: &str) -> super::PlatformResult<()> {
    wallpaper::set_from_path(path).map_err(|e| format!("[linux] set wallpaper failed: {e}"))?;
    wallpaper::set_mode(wallpaper::Mode::Crop)
      .map_err(|e| format!("[linux] set mode failed: {e}"))?;
    Ok(())
  }
}
