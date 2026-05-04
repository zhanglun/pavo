use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app
      .dialog()
      .message("A new version is available. Install now?")
      .title("Update Available")
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
