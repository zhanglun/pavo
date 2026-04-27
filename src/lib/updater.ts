import { check } from "@tauri-apps/plugin-updater";
import { message } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";

export async function checkUpdate() {
  try {
    const update = await check();

    if (update) {
      console.log(
        `found update ${update.version} from ${update.date} with notes ${update.body}`
      );
      let downloaded = 0;
      let contentLength = 0;
      // alternatively we could also call update.download() and update.install() separately
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength || 0;
            console.log(
              `started downloading ${event.data.contentLength} bytes`
            );
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            console.log(`downloaded ${downloaded} from ${contentLength}`);
            break;
          case "Finished":
            console.log("download finished");
            break;
        }
      });

      console.log("update installed");
      await relaunch();
    } else {
      await message("当前已是最新版本", { title: "Pavo", kind: "info" });
    }
  } catch (err) {
    await message("检查更新失败，请稍后再试", { title: "Pavo", kind: "error" });
  }
}
