//! Rust 侧用户可见文案的语言适配。
//!
//! 桌面应用惯例：跟随系统显示语言，语言变更在应用重启后生效。
//! 目前覆盖托盘菜单与更新提示弹窗；未知 locale 一律回落英文。

use sys_locale::get_locale;

/// 支持的语言。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
  Chinese,
  English,
}

impl Language {
  /// 从 BCP-47 locale（如 zh-CN / en-US）解析；zh 系（含繁体地区）归入中文，其余回落英文。
  pub fn from_locale(locale: Option<&str>) -> Self {
    match locale {
      Some(locale) if locale.to_ascii_lowercase().starts_with("zh") => Self::Chinese,
      _ => Self::English,
    }
  }

  /// 跟随系统显示语言。
  pub fn system() -> Self {
    Self::from_locale(get_locale().as_deref())
  }
}

/// 托盘菜单文案。菜单项 id 与文案解耦（id 稳定、标签可换语言）。
pub struct TrayTexts {
  pub quit: &'static str,
  pub show: &'static str,
  pub hide: &'static str,
  pub previous_wallpaper: &'static str,
  pub next_wallpaper: &'static str,
  pub check_for_updates: &'static str,
  pub settings: &'static str,
}

pub fn tray_texts(language: Language) -> TrayTexts {
  match language {
    Language::Chinese => TrayTexts {
      quit: "退出",
      show: "显示",
      hide: "隐藏",
      previous_wallpaper: "上一张壁纸",
      next_wallpaper: "下一张壁纸",
      check_for_updates: "检查更新",
      settings: "设置…",
    },
    Language::English => TrayTexts {
      quit: "Quit",
      show: "Show",
      hide: "Hide",
      previous_wallpaper: "Previous Wallpaper",
      next_wallpaper: "Next Wallpaper",
      check_for_updates: "Check for Updates",
      settings: "Settings...",
    },
  }
}

/// 更新提示弹窗文案（标题 + 正文）。
pub fn update_dialog_texts(language: Language) -> (&'static str, &'static str) {
  match language {
    Language::Chinese => ("发现新版本", "发现新版本，是否立即安装？"),
    Language::English => ("Update Available", "A new version is available. Install now?"),
  }
}

#[cfg(test)]
mod tests {
  use super::{tray_texts, update_dialog_texts, Language};

  #[test]
  fn locale_parsing_maps_zh_family_to_chinese() {
    assert_eq!(Language::from_locale(Some("zh-CN")), Language::Chinese);
    assert_eq!(Language::from_locale(Some("ZH-TW")), Language::Chinese);
    assert_eq!(Language::from_locale(Some("en-US")), Language::English);
    assert_eq!(Language::from_locale(Some("fr-FR")), Language::English);
    assert_eq!(Language::from_locale(None), Language::English);
  }

  #[test]
  fn tray_texts_cover_every_menu_entry_in_both_languages() {
    for language in [Language::Chinese, Language::English] {
      let texts = tray_texts(language);
      for text in [texts.quit, texts.show, texts.hide, texts.previous_wallpaper, texts.next_wallpaper, texts.check_for_updates, texts.settings] {
        assert!(!text.is_empty(), "tray label must not be empty for {language:?}");
      }
    }
    let chinese = tray_texts(Language::Chinese);
    assert_eq!(chinese.quit, "退出");
    assert_eq!(chinese.next_wallpaper, "下一张壁纸");
    let english = tray_texts(Language::English);
    assert_eq!(english.settings, "Settings...");
  }

  #[test]
  fn update_dialog_texts_cover_both_languages() {
    assert_eq!(update_dialog_texts(Language::Chinese).0, "发现新版本");
    assert_eq!(update_dialog_texts(Language::English).1, "A new version is available. Install now?");
  }
}
