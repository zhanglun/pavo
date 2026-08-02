// 统计 Pavo 的真实下载量（过滤 latest.json / *.sig 等更新器元数据）与 stars，
// 写入 website/public/stats.json 供官网展示。
//
// 由 .github/workflows/update-stats.yml 定时调用；用 GITHUB_TOKEN 认证调用
// GitHub API（5000/小时），规避未认证的 60/小时 限流。
import { writeFileSync, readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, "..");
const OUT = resolve(root, "website/public/stats.json");

const REPO = process.env.GITHUB_REPO || "zhanglun/pavo";
const TOKEN = process.env.GITHUB_TOKEN;

// 只统计真实安装包，排除 updater 元数据：
//   - latest.json：应用每次启动检查更新都会下载，计数虚高，必须排除
//   - *.sig：更新时下载的签名文件，非真实安装下载
const PKG_EXT = [".deb", ".rpm", ".appimage", ".dmg", ".exe", ".msi"];

const headers = {
  Accept: "application/vnd.github+json",
  "X-GitHub-Api-Version": "2022-11-28",
  ...(TOKEN ? { Authorization: `Bearer ${TOKEN}` } : {}),
};

async function gh(url) {
  const res = await fetch(url, { headers });
  if (!res.ok) throw new Error(`${res.status} ${url}: ${await res.text()}`);
  return res.json();
}

async function fetchReleases() {
  const all = [];
  for (let page = 1; page <= 10; page++) {
    const data = await gh(
      `https://api.github.com/repos/${REPO}/releases?per_page=100&page=${page}`
    );
    all.push(...data);
    if (data.length < 100) break;
  }
  return all;
}

function isPackage(name) {
  const lower = name.toLowerCase();
  return PKG_EXT.some((ext) => lower.endsWith(ext));
}

function countDownloads(releases) {
  let total = 0;
  let meta = 0;
  for (const r of releases) {
    for (const a of r.assets || []) {
      if (isPackage(a.name)) total += a.download_count;
      else meta += a.download_count;
    }
  }
  return { total, meta };
}

async function main() {
  const [releases, repo] = await Promise.all([
    fetchReleases(),
    gh(`https://api.github.com/repos/${REPO}`),
  ]);
  const { total, meta } = countDownloads(releases);
  const stats = {
    downloads: total,
    stars: repo.stargazers_count ?? 0,
    updatedAt: new Date().toISOString(),
  };
  console.log(`真实安装包下载: ${total}（已排除元数据 latest.json/.sig 共 ${meta}）`);
  console.log(`stars: ${stats.stars}`);

  // 数值未变化时不写文件，避免每天产生空 commit（updatedAt 反映最后变化时刻）
  let prev = null;
  try {
    prev = JSON.parse(readFileSync(OUT, "utf-8"));
  } catch {
    /* 首次创建 */
  }
  if (prev && prev.downloads === stats.downloads && prev.stars === stats.stars) {
    console.log("数值未变化，跳过写入");
    return;
  }
  writeFileSync(OUT, JSON.stringify(stats, null, 2) + "\n");
  console.log(`已写入 ${OUT}`);
}

main().catch((e) => {
  console.error(e.message);
  process.exit(1);
});
