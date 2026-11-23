//! 系统壁纸设置平台层。
//!
//! 该层是"应用到系统"的唯一出口：Windows 走 IDesktopWallpaper（Win8+ 逐屏接口），
//! macOS 走 NSWorkspace 逐屏 API，Linux 保留 wallpaper crate 的 DE 命令实现。
//! 语义对齐旧实现：所有屏幕设为同一张图 + 填充（Fill）模式。
//!
//! 屏幕标识为运行时标识（每次 list_monitors 实时枚举产生），不持久化。

use std::collections::HashSet;
use std::path::Path;
use std::sync::OnceLock;
use serde::Serialize;

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

pub type PlatformResult<T> = Result<T, String>;

/// 屏幕描述。id 为运行时标识（本次枚举产生，前端会话内回传；不持久化）。
/// current_filename 为 OS 反查的当前壁纸文件名。
#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
  pub id: String,
  pub name: String,
  pub index: usize,
  pub width: u32,
  pub height: u32,
  pub is_primary: bool,
  pub current_filename: Option<String>,
}

/// 枚举快照：能力位 + 屏幕列表，单次 IPC 同时返回。
/// per_monitor 为 false 时（Linux），逐屏命令返回错误。
#[derive(Debug, Clone, Serialize)]
pub struct MonitorSnapshot {
  pub per_monitor: bool,
  pub monitors: Vec<MonitorInfo>,
}

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

/// 从本地路径提取文件名（用于 OS 反查结果与相册数据映射）。
pub fn filename_from_path(path: &str) -> Option<String> {
  Path::new(path)
    .file_name()
    .and_then(|name| name.to_str())
    .map(|name| name.to_string())
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

/// 为指定屏幕设置壁纸（填充模式）。id 必须来自同一会话内的 list_monitors 枚举结果。
pub fn set_wallpaper_for(monitor_id: &str, path: &str) -> PlatformResult<()> {
  #[cfg(target_os = "windows")]
  return windows_impl::set_wallpaper_for(monitor_id, path);
  #[cfg(target_os = "macos")]
  return macos_impl::set_wallpaper_for(monitor_id, path);
  #[cfg(target_os = "linux")]
  return linux_impl::set_wallpaper_for(monitor_id);
}

/// 枚举显示器并反查每屏当前壁纸。
pub fn list_monitors() -> PlatformResult<MonitorSnapshot> {
  #[cfg(target_os = "windows")]
  return windows_impl::list_monitors();
  #[cfg(target_os = "macos")]
  return macos_impl::list_monitors();
  #[cfg(target_os = "linux")]
  return linux_impl::list_monitors();
}

/// 当前被任一屏幕引用的壁纸文件名集合（clean_cache 引用保护用）。
pub fn referenced_wallpaper_files() -> HashSet<String> {
  list_monitors()
    .map(|snapshot| {
      snapshot
        .monitors
        .iter()
        .filter_map(|monitor| monitor.current_filename.clone())
        .collect()
    })
    .unwrap_or_default()
}

#[cfg(target_os = "windows")]
mod windows_impl {
  use windows::core::{HSTRING, PCWSTR, PWSTR};
  use windows::Win32::Foundation::{BOOL, LPARAM, RECT, RPC_E_CHANGED_MODE};
  use windows::Win32::Graphics::Gdi::{
    EnumDisplayDevicesW, EnumDisplayMonitors, EnumDisplaySettingsW, GetMonitorInfoW, DEVMODEW,
    DISPLAY_DEVICEW, DISPLAY_DEVICE_PRIMARY_DEVICE, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW,
    ENUM_CURRENT_SETTINGS,
  };
  use windows::Win32::UI::WindowsAndMessaging::{
    EDD_GET_DEVICE_INTERFACE_NAME, MONITORINFOF_PRIMARY,
  };
  use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
    COINIT_APARTMENTTHREADED,
  };
  use windows::Win32::UI::Shell::{DesktopWallpaper, DWPOS_FILL, IDesktopWallpaper};

  /// 在独立的 COM 初始化作用域内访问 IDesktopWallpaper。
  /// op 内部自行映射错误信息；作用域保证 CoUninitialize 与初始化配对。
  unsafe fn with_desktop_wallpaper<T>(
    op: impl FnOnce(&IDesktopWallpaper) -> super::PlatformResult<T>,
  ) -> super::PlatformResult<T> {
    let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    if hr.is_err() && hr != RPC_E_CHANGED_MODE {
      return Err(format!("[win] CoInitializeEx failed: {hr}"));
    }
    let need_uninit = hr.is_ok();

    let result = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)
      .map_err(|e| format!("[win] create DesktopWallpaper failed: {e}"))
      .and_then(|wallpaper| op(&wallpaper));

    if need_uninit {
      CoUninitialize();
    }
    result
  }

  /// COM 分配的 PWSTR 转 String 并释放。
  fn take_pwstr(pw: PWSTR) -> Option<String> {
    if pw.is_null() {
      return None;
    }
    let text = unsafe { pw.to_string() }.ok();
    unsafe { CoTaskMemFree(Some(pw.0.cast())) };
    text
  }

  fn utf16_to_string(buf: &[u16]) -> Option<String> {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16(&buf[..end]).ok()
  }

  /// 与 IDesktopWallpaper 的设备路径关联的显示器附加信息。
  struct MonitorDeviceInfo {
    /// EDID 友好名称
    name: Option<String>,
    /// 适配器设备名（形如 \\.\DISPLAY1），用于 EnumDisplaySettingsW 兜底
    adapter_device_name: Option<String>,
    /// 适配器带主屏标志
    is_primary: bool,
  }

  /// 按 IDesktopWallpaper 的设备路径匹配显示器，取 EDID 名称、适配器设备名与主屏标志。
  fn monitor_device_info(monitor_id: &str) -> Option<MonitorDeviceInfo> {
    unsafe {
      let mut adapter_index = 0u32;
      loop {
        let mut adapter = DISPLAY_DEVICEW::default();
        adapter.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;
        if !EnumDisplayDevicesW(PCWSTR::null(), adapter_index, &mut adapter, 0).as_bool() {
          return None;
        }
        adapter_index += 1;

        let adapter_name = utf16_to_string(&adapter.DeviceName).unwrap_or_default();
        let is_primary = adapter.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE != 0;

        // 一个适配器可能挂接多块显示器，逐一比对设备路径
        let mut monitor_index = 0u32;
        loop {
          let mut monitor = DISPLAY_DEVICEW::default();
          monitor.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;
          if !EnumDisplayDevicesW(
            PCWSTR::from_raw(adapter.DeviceName.as_ptr()),
            monitor_index,
            &mut monitor,
            EDD_GET_DEVICE_INTERFACE_NAME,
          )
          .as_bool()
          {
            break;
          }
          monitor_index += 1;

          let device_id = utf16_to_string(&monitor.DeviceID).unwrap_or_default();
          if device_id.eq_ignore_ascii_case(monitor_id) {
            return Some(MonitorDeviceInfo {
              name: utf16_to_string(&monitor.DeviceString),
              adapter_device_name: if adapter_name.is_empty() {
                None
              } else {
                Some(adapter_name)
              },
              is_primary,
            });
          }
        }
      }
    }
  }

  /// 通过适配器设备名取当前分辨率（GetMonitorRECT 失败时的兜底）。
  fn adapter_resolution(adapter_device_name: &str) -> Option<(u32, u32)> {
    unsafe {
      let wide = HSTRING::from(adapter_device_name);
      let mut devmode = DEVMODEW::default();
      devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
      let ok = EnumDisplaySettingsW(
        PCWSTR::from_raw(wide.as_ptr()),
        ENUM_CURRENT_SETTINGS,
        &mut devmode,
      )
      .as_bool();
      if ok && devmode.dmPelsWidth > 0 && devmode.dmPelsHeight > 0 {
        Some((devmode.dmPelsWidth, devmode.dmPelsHeight))
      } else {
        None
      }
    }
  }

  /// 全系统当前活跃的物理显示器（虚拟桌面）。克隆模式下只会有一个条目。
  fn active_monitors() -> Vec<(RECT, bool)> {
    unsafe extern "system" fn callback(
      hmonitor: HMONITOR,
      _hdc: HDC,
      rect: *mut RECT,
      lparam: LPARAM,
    ) -> BOOL {
      let out = unsafe { &mut *(lparam.0 as *mut Vec<(RECT, bool)>) };
      let mut info = MONITORINFOEXW::default();
      info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
      let ok = unsafe {
        GetMonitorInfoW(hmonitor, &mut info as *mut MONITORINFOEXW as *mut MONITORINFO).as_bool()
      };
      if ok {
        out.push((
          unsafe { *rect },
          info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY != 0,
        ));
      }
      true.into()
    }

    let mut out: Vec<(RECT, bool)> = Vec::new();
    unsafe {
      // 枚举起点调用本身不会失败，结果由回调逐条收集
      let _ = EnumDisplayMonitors(
        HDC(std::ptr::null_mut()),
        None,
        Some(callback),
        LPARAM(&mut out as *mut _ as isize),
      );
    }
    out
  }

  /// 分辨率与主屏的多级兜底（真实机器上 GetMonitorRECT 可能对个别屏返回 E_FAIL）：
  /// 1. GetMonitorRECT（绝大多数环境有效）
  /// 2. 适配器匹配 + EnumDisplaySettingsW
  /// 3. 系统仅报告一块活跃物理显示器（克隆模式）时，共享其几何
  /// 4. 0×0 + 非主屏（UI 显示未知分辨率，逐屏设置能力不受影响）
  unsafe fn resolve_geometry(
    wallpaper: &IDesktopWallpaper,
    id: &str,
    device_info: Option<&MonitorDeviceInfo>,
    actives: &[(RECT, bool)],
  ) -> (u32, u32, bool) {
    let mut width = 0u32;
    let mut height = 0u32;
    let mut origin_primary = false;

    if let Ok(rect) = wallpaper.GetMonitorRECT(&HSTRING::from(id)) {
      if rect.right > rect.left && rect.bottom > rect.top {
        width = (rect.right - rect.left) as u32;
        height = (rect.bottom - rect.top) as u32;
      }
      origin_primary = rect.left == 0 && rect.top == 0;
    }

    if width == 0 {
      if let Some(res) = device_info
        .and_then(|info| info.adapter_device_name.as_deref())
        .and_then(adapter_resolution)
      {
        width = res.0;
        height = res.1;
      }
    }

    if width == 0 && actives.len() == 1 {
      let (rect, primary) = &actives[0];
      width = (rect.right - rect.left).max(0) as u32;
      height = (rect.bottom - rect.top).max(0) as u32;
      origin_primary = *primary;
    }

    let is_primary = device_info
      .map(|info| info.is_primary)
      .unwrap_or(origin_primary);
    (width, height, is_primary)
  }

  /// 查询某屏当前壁纸的完整路径（端到端测试的取证与恢复用）。
  #[cfg(test)]
  pub fn current_wallpaper_path(monitor_id: &str) -> super::PlatformResult<String> {
    unsafe {
      with_desktop_wallpaper(|wallpaper| {
        let path = wallpaper
          .GetWallpaper(&HSTRING::from(monitor_id))
          .map_err(|e| format!("[win] GetWallpaper failed: {e}"))?;
        take_pwstr(path)
          .filter(|p| !p.is_empty())
          .ok_or_else(|| format!("[win] no wallpaper set for monitor: {monitor_id}"))
      })
    }
  }

  pub fn set_wallpaper_all(path: &str) -> super::PlatformResult<()> {
    unsafe {
      with_desktop_wallpaper(|wallpaper| {
        // monitorID 传 NULL 时 SetWallpaper 作用于所有显示器；SetPosition 为全局填充语义
        wallpaper
          .SetWallpaper(PCWSTR::null(), &HSTRING::from(path))
          .map_err(|e| format!("[win] SetWallpaper failed: {e}"))?;
        wallpaper
          .SetPosition(DWPOS_FILL)
          .map_err(|e| format!("[win] SetPosition failed: {e}"))?;
        Ok(())
      })
    }
  }

  pub fn set_wallpaper_for(monitor_id: &str, path: &str) -> super::PlatformResult<()> {
    unsafe {
      with_desktop_wallpaper(|wallpaper| {
        let count = wallpaper
          .GetMonitorDevicePathCount()
          .map_err(|e| format!("[win] GetMonitorDevicePathCount failed: {e}"))?;
        for index in 0..count {
          let id = match take_pwstr(
            wallpaper
              .GetMonitorDevicePathAt(index)
              .map_err(|e| format!("[win] GetMonitorDevicePathAt({index}) failed: {e}"))?,
          ) {
            Some(id) => id,
            None => continue,
          };
          if id.eq_ignore_ascii_case(monitor_id) {
            return wallpaper
              .SetWallpaper(&HSTRING::from(id.as_str()), &HSTRING::from(path))
              .map_err(|e| format!("[win] SetWallpaper failed: {e}"))
              .and_then(|()| {
                wallpaper
                  .SetPosition(DWPOS_FILL)
                  .map_err(|e| format!("[win] SetPosition failed: {e}"))
              });
          }
        }
        Err(format!("[win] monitor not found: {monitor_id}"))
      })
    }
  }

  pub fn list_monitors() -> super::PlatformResult<super::MonitorSnapshot> {
    unsafe {
      with_desktop_wallpaper(|wallpaper| {
        let count = wallpaper
          .GetMonitorDevicePathCount()
          .map_err(|e| format!("[win] GetMonitorDevicePathCount failed: {e}"))?;

        let mut monitors = Vec::with_capacity(count as usize);
        let actives = active_monitors();
        for index in 0..count {
          let id = match take_pwstr(
            wallpaper
              .GetMonitorDevicePathAt(index)
              .map_err(|e| format!("[win] GetMonitorDevicePathAt({index}) failed: {e}"))?,
          ) {
            Some(id) => id,
            None => continue,
          };

          // GetMonitorRECT 在部分驱动/显示器组合上返回 E_FAIL（真实机器上已复现），
          // 不能因此让整个枚举失败：分辨率与主屏走多级兜底
          let device_info = monitor_device_info(&id);
          let (width, height, is_primary) =
            resolve_geometry(wallpaper, &id, device_info.as_ref(), &actives);

          let current_filename = wallpaper
            .GetWallpaper(&HSTRING::from(id.as_str()))
            .ok()
            .and_then(take_pwstr)
            .and_then(|path| super::filename_from_path(&path));

          let name = device_info
            .and_then(|info| info.name)
            .unwrap_or_else(|| format!("显示器 {}", index + 1));

          monitors.push(super::MonitorInfo {
            id,
            name,
            index: index as usize,
            width,
            height,
            is_primary,
            current_filename,
          });
        }
        Ok(super::MonitorSnapshot {
          per_monitor: true,
          monitors,
        })
      })
    }
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
  use objc2_foundation::{NSNumber, NSDictionary, NSString, NSURL};
  use std::sync::mpsc;
  use std::time::Duration;

  /// 主线程任务的通用执行入口：dispatch + 超时兜底。
  fn run_on_main<T>(
    task: impl FnOnce() -> super::PlatformResult<T> + Send + 'static,
  ) -> super::PlatformResult<T> {
    let handle = super::app_handle()?;
    let (tx, rx) = mpsc::channel::<super::PlatformResult<T>>();

    handle
      .run_on_main_thread(move || {
        let result = autoreleasepool(task);
        let _ = tx.send(result);
      })
      .map_err(|e| format!("[mac] dispatch to main thread failed: {e}"))?;

    // 主线程被阻塞时兜底超时，避免调用线程永久挂起
    match rx.recv_timeout(Duration::from_secs(10)) {
      Ok(result) => result,
      Err(_) => Err("[mac] wallpaper task did not complete in time".to_string()),
    }
  }

  unsafe fn main_thread_marker() -> super::PlatformResult<MainThreadMarker> {
    MainThreadMarker::new()
      .ok_or_else(|| "[mac] not running on main thread".to_string())
  }

  /// 构造填充缩放选项：保持与旧实现的视觉语义一致（按比例放大/缩小填满屏幕）。
  unsafe fn build_fill_options() -> Retained<NSDictionary<NSWorkspaceDesktopImageOptionKey, AnyObject>> {
    let scaling = NSNumber::new_isize(NSImageScaling::NSImageScaleProportionallyUpOrDown.0);
    let any: Retained<AnyObject> = scaling.into_super();
    NSDictionary::from_retained_objects(&[NSWorkspaceDesktopImageScalingKey], &[any])
  }

  /// screen 的运行时唯一标识（CGDirectDisplayID）。
  unsafe fn screen_display_id(screen: &NSScreen) -> super::PlatformResult<u32> {
    let key = NSString::from_str("NSScreenNumber");
    let number = screen
      .deviceDescription()
      .objectForKey(&key)
      .ok_or_else(|| "[mac] screen missing NSScreenNumber".to_string())?
      .downcast::<NSNumber>()
      .map_err(|_| "[mac] NSScreenNumber is not a NSNumber".to_string())?;
    Ok(number.as_i32() as u32)
  }

  /// 遍历屏幕执行逐屏动作，所有屏幕场景复用同一 url 与 options。
  unsafe fn for_each_screen(
    mut action: impl FnMut(&NSScreen) -> super::PlatformResult<()>,
  ) -> super::PlatformResult<()> {
    let mtm = main_thread_marker()?;
    let screens = NSScreen::screens(mtm);
    for screen in screens.to_vec().iter() {
      action(screen)?;
    }
    Ok(())
  }

  pub fn set_wallpaper_all(path: &str) -> super::PlatformResult<()> {
    run_on_main(move || unsafe {
      let url = NSURL::from_file_path(Path::new(path))
        .ok_or_else(|| format!("[mac] invalid wallpaper path: {path}"))?;
      let workspace = NSWorkspace::sharedWorkspace();
      let options = build_fill_options();

      // 遍历所有屏幕逐屏设置，与旧 osascript "tell every desktop" 行为一致
      for_each_screen(|screen| {
        workspace
          .setDesktopImageURL_forScreen_options_error(&url, screen, &options)
          .map_err(|e| format!("[mac] set desktop image failed: {e:?}"))
      })
    })
  }

  pub fn set_wallpaper_for(monitor_id: &str, path: &str) -> super::PlatformResult<()> {
    run_on_main(move || unsafe {
      let target = monitor_id
        .parse::<u32>()
        .map_err(|_| format!("[mac] monitor not found: {monitor_id}"))?;
      let url = NSURL::from_file_path(Path::new(path))
        .ok_or_else(|| format!("[mac] invalid wallpaper path: {path}"))?;
      let workspace = NSWorkspace::sharedWorkspace();
      let options = build_fill_options();

      let mut found = false;
      for_each_screen(|screen| {
        if screen_display_id(screen)? == target {
          found = true;
          workspace
            .setDesktopImageURL_forScreen_options_error(&url, screen, &options)
            .map_err(|e| format!("[mac] set desktop image failed: {e:?}"))
        } else {
          Ok(())
        }
      })?;

      if found {
        Ok(())
      } else {
        Err(format!("[mac] monitor not found: {monitor_id}"))
      }
    })
  }

  pub fn list_monitors() -> super::PlatformResult<super::MonitorSnapshot> {
    run_on_main(|| unsafe {
      let mtm = main_thread_marker()?;
      let screens = NSScreen::screens(mtm);
      let workspace = NSWorkspace::sharedWorkspace();

      let mut monitors = Vec::new();
      for (index, screen) in screens.to_vec().iter().enumerate() {
        let id = screen_display_id(screen)?.to_string();
        let name = screen.localizedDescription().to_string();
        let frame = screen.frame();
        let scale = screen.backingScaleFactor();
        let width = (frame.size.width * scale).round() as u32;
        let height = (frame.size.height * scale).round() as u32;
        // NSScreen 数组首元素即主屏
        let is_primary = index == 0;
        let current_filename = workspace
          .desktopImageURLForScreen(screen)
          .and_then(|url| url.path())
          .and_then(|path| path.to_string())
          .and_then(|path| super::filename_from_path(&path));

        monitors.push(super::MonitorInfo {
          id,
          name,
          index,
          width,
          height,
          is_primary,
          current_filename,
        });
      }
      Ok(super::MonitorSnapshot {
        per_monitor: true,
        monitors,
      })
    })
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

  pub fn set_wallpaper_for(monitor_id: &str) -> super::PlatformResult<()> {
    let _ = monitor_id;
    Err("[linux] per-monitor wallpaper is not supported on this system".to_string())
  }

  pub fn list_monitors() -> super::PlatformResult<super::MonitorSnapshot> {
    Ok(super::MonitorSnapshot {
      per_monitor: false,
      monitors: vec![],
    })
  }
}

#[cfg(test)]
mod tests {
  use super::filename_from_path;

  #[test]
  fn filename_from_path_extracts_last_segment() {
    assert_eq!(
      filename_from_path(r"C:\Users\zhang\.pavo\OHR.Foo_CN-CN123_UHD.jpg"),
      Some("OHR.Foo_CN-CN123_UHD.jpg".to_string())
    );
    assert_eq!(
      filename_from_path("/home/user/.pavo/OHR.Bar.jpg"),
      Some("OHR.Bar.jpg".to_string())
    );
  }

  #[test]
  fn filename_from_path_returns_none_without_filename() {
    assert_eq!(filename_from_path(""), None);
    assert_eq!(filename_from_path(r"C:\"), None);
    assert_eq!(filename_from_path("/"), None);
  }

  // 真机集成验证：枚举当前显示器并检查基本健全性（Windows 本机可跑）
  #[cfg(target_os = "windows")]
  #[test]
  fn list_monitors_returns_connected_displays() {
    let snapshot = super::list_monitors().expect("list_monitors should succeed");
    assert!(snapshot.per_monitor);
    assert!(
      !snapshot.monitors.is_empty(),
      "dev machine should have at least one display"
    );
    for monitor in &snapshot.monitors {
      assert!(!monitor.id.is_empty());
      assert!(monitor.width > 0 && monitor.height > 0);
    }
    assert!(
      snapshot.monitors.iter().any(|monitor| monitor.is_primary),
      "exactly the origin monitor is primary"
    );
  }

  // 无效 id 应该报错且不触碰壁纸（只走枚举分支）
  #[cfg(target_os = "windows")]
  #[test]
  fn set_wallpaper_for_unknown_monitor_fails() {
    let err = super::windows_impl::set_wallpaper_for(r"\\?\DISPLAY#NOPE#NOPE", "unused.jpg")
      .expect_err("unknown monitor must fail");
    assert!(err.contains("monitor not found"));
  }
}



/// 端到端（真机，会短暂修改真实桌面并在结束时恢复原壁纸）：
/// cargo test e2e_wallpaper -- --ignored --nocapture
///
/// 注意：测试图必须是真实有效的图片——无效图片（如损坏的字节流）会被 Windows
/// 静默拒绝渲染并让 GetWallpaper 进入空状态。这里复制应用图标作为测试图。
#[cfg(all(test, target_os = "windows"))]
mod e2e_tests {
  fn prepare_test_image() -> std::path::PathBuf {
    // 以仓库中的真实 PNG（应用图标）为源，复制到应用缓存目录
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = manifest.join("..").join("public").join("icon.png");
    let app_folder = crate::config::PavoConfig::get_app_folder().expect("app folder");
    let target = std::path::Path::new(&app_folder).join("pavo-e2e-test.png");
    std::fs::copy(&source, &target).expect("copy icon.png as test image");
    target
  }

  #[test]
  #[ignore = "端到端：短暂修改真实桌面壁纸并自动恢复，仅本机手动执行"]
  fn e2e_wallpaper_pipeline_sets_targets_and_restores() {
    let snapshot = super::list_monitors().expect("enumerate monitors");
    assert!(snapshot.per_monitor, "per-monitor capability required");
    assert!(!snapshot.monitors.is_empty(), "at least one display");

    // 取证：每屏原始壁纸完整路径（壁纸应用是异步的，取证与断言都需要轮询容错）
    let original: Vec<(String, String)> = snapshot
      .monitors
      .iter()
      .filter_map(|monitor| {
        super::windows_impl::current_wallpaper_path(&monitor.id)
          .ok()
          .map(|path| (monitor.id.clone(), path))
      })
      .collect();
    assert!(!original.is_empty(), "monitors should have current wallpapers");
    println!("original wallpapers: {original:?}");

    let test_image = prepare_test_image();
    let test_path = test_image.to_str().unwrap().to_string();

    // 步骤 1：所有屏幕 → 测试图
    super::windows_impl::set_wallpaper_all(&test_path).expect("set all screens");
    for (id, _) in &original {
      poll_wallpaper(id, &test_path, "set all");
    }
    println!("step 1 ok: all screens -> test image");

    // 步骤 2：逐屏定向 → 只改第一块屏为原始壁纸，其余屏不受影响
    let (first_id, first_original) = &original[0];
    super::windows_impl::set_wallpaper_for(first_id, first_original).expect("targeted set");
    poll_wallpaper(first_id, first_original, "targeted set");
    if let Some((second_id, _)) = original.get(1) {
      poll_wallpaper(second_id, &test_path, "targeted set isolation");
    }
    println!("step 2 ok: targeted set only affects the chosen screen");

    // 步骤 3：恢复所有屏幕的原始壁纸
    for (id, path) in &original {
      super::windows_impl::set_wallpaper_for(id, path).expect("restore");
      poll_wallpaper(id, path, "restore");
    }
    println!("step 3 ok: all screens restored to original wallpapers");

    let _ = std::fs::remove_file(&test_image);
  }

  /// 轮询等待 GetWallpaper 落到期望值；超时带实际值失败。
  fn poll_wallpaper(id: &str, expected: &str, label: &str) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
      match super::windows_impl::current_wallpaper_path(id) {
        Ok(path) if path == expected => return,
        Ok(path) if std::time::Instant::now() >= deadline => {
          panic!("{label}: wallpaper did not settle: expected {expected:?}, got {path:?}");
        }
        Err(_) if std::time::Instant::now() >= deadline => {
          panic!("{label}: wallpaper did not settle to {expected:?} (query empty)");
        }
        _ => std::thread::sleep(std::time::Duration::from_millis(250)),
      }
    }
  }
}


/// 逐屏独立轮播的真机端到端（会短暂修改真实桌面并自动恢复）：
/// 预设两块屏为相册中不同的两张图 → 推进一轮 → 验证各自独立 +1 → 恢复原壁纸。
/// cargo test e2e_per_monitor_rotation -- --ignored --nocapture
#[cfg(all(test, target_os = "windows"))]
mod e2e_rotation_tests {
  use crate::config::RotateMode;
  use crate::services::bing;
  use crate::scheduler;
  use std::time::{Duration, Instant};

  /// 轮询某屏的当前壁纸文件名落到期望值（壁纸应用是异步的）。
  async fn poll_filename(monitor_id: &str, expected: &str, label: &str) {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
      let actual = crate::platform::list_monitors()
        .ok()
        .and_then(|snapshot| {
          snapshot
            .monitors
            .iter()
            .find(|m| m.id == monitor_id)
            .and_then(|m| m.current_filename.clone())
        });
      match actual {
        Some(name) if name == expected => return,
        other if Instant::now() >= deadline => {
          panic!("{label}: wallpaper did not settle: expected {expected:?}, got {other:?}");
        }
        _ => tokio::time::sleep(Duration::from_millis(300)).await,
      }
    }
  }

  #[tokio::test]
  #[ignore = "端到端：短暂修改真实桌面壁纸并自动恢复，仅本机手动执行"]
  async fn e2e_per_monitor_rotation_advances_each_screen_independently() {
    let app = tauri::test::mock_app();
    let handle = app.handle().clone();

    let snapshot = crate::platform::list_monitors().expect("enumerate monitors");
    assert!(snapshot.per_monitor, "per-monitor capability required");
    assert!(
      snapshot.monitors.len() >= 2,
      "per-monitor rotation E2E requires at least two displays"
    );

    // 取证：每屏原始壁纸完整路径，结束后恢复
    let original: Vec<(String, String)> = snapshot
      .monitors
      .iter()
      .filter_map(|monitor| {
        super::windows_impl::current_wallpaper_path(&monitor.id)
          .ok()
          .map(|path| (monitor.id.clone(), path))
      })
      .collect();
    assert!(original.len() >= 2, "monitors should have current wallpapers");

    let mut scheduler = scheduler::SCHEDULER.lock().await;
    let list = scheduler.batch_fetch().await.expect("fetch album list");
    assert!(
      list.len() >= 3,
      "album list should have at least three photos to verify independent advance"
    );

    // 预设：屏 1 ← 列表第 0 张，屏 2 ← 列表第 1 张（两张不同，为独立性验证铺底）
    let screen_a = snapshot.monitors[0].id.clone();
    let screen_b = snapshot.monitors[1].id.clone();
    bing::Wallpaper::set_wallpaper(&list[0].urls[0], Some(&screen_a))
      .await
      .expect("preset screen a");
    bing::Wallpaper::set_wallpaper(&list[1].urls[0], Some(&screen_b))
      .await
      .expect("preset screen b");
    poll_filename(&screen_a, &list[0].filename, "preset screen a").await;
    poll_filename(&screen_b, &list[1].filename, "preset screen b").await;

    // 推进一轮：每屏应从各自当前位置 +1（环绕），互不影响
    let expected_a = &list[scheduler::Scheduler::next_position(0, list.len(), 1)].filename;
    let expected_b = &list[scheduler::Scheduler::next_position(1, list.len(), 1)].filename;
    scheduler
      .advance_per_monitor(&handle, &RotateMode::Sequential, 1)
      .await
      .expect("advance per monitor");

    poll_filename(&screen_a, expected_a, "advance screen a").await;
    poll_filename(&screen_b, expected_b, "advance screen b").await;
    println!("per-monitor advance ok: {screen_a} -> {expected_a:?}, {screen_b} -> {expected_b:?}");

    // 恢复每屏原始壁纸
    drop(scheduler);
    for (id, path) in &original {
      super::windows_impl::set_wallpaper_for(id, path).expect("restore");
    }
    for (id, path) in &original {
      let deadline = Instant::now() + Duration::from_secs(10);
      loop {
        let restored = super::windows_impl::current_wallpaper_path(id)
          .ok()
          .map(|p| p == *path)
          .unwrap_or(false);
        if restored || Instant::now() >= deadline {
          assert!(restored, "screen {id} should be restored");
          break;
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
      }
    }
    println!("original wallpapers restored");
  }
}
