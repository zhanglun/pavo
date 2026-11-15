use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

/// 检查并安装更新。弹窗文案由调用方（bin）按语言提供，lib 保持无语言感知。
pub async fn update(app: tauri::AppHandle, title: &str, message: &str) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app
      .dialog()
      .message(message)
      .title(title)
      .buttons(MessageDialogButtons::OkCancel)
      .show(move |confirmed| {
        let _ = tx.send(confirmed);
      });

    let confirmed = rx.await.unwrap_or(false);
    if !confirmed {
      return Ok(());
    }

    let mut downloaded = 0;

    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          log::info!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          log::info!("download finished");
        },
      )
      .await?;

    log::info!("update installed");
    app.restart();
  }

  Ok(())
}
