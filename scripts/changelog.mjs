// 轻量的 conventional-changelog 实现：解析 `type(scope)!: subject` 格式的
// git 提交，生成与现有 CHANGELOG.md（standard-version angular 风格）一致的条目。
// 零依赖，格式完全可控。
import { execSync } from "node:child_process";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, "..");

const TYPE_TITLES = {
  feat: "Features",
  fix: "Bug Fixes",
  perf: "Performance Improvements",
};
// 展示顺序：新功能 → 性能 → 修复
const TYPE_ORDER = ["feat", "perf", "fix"];

function run(cmd) {
  return execSync(cmd, { cwd: root }).toString();
}

function repoUrl() {
  try {
    let url = run("git remote get-url origin").trim();
    url = url.replace(/\.git$/, "");
    if (url.startsWith("git@")) {
      // git@github.com:owner/repo → https://github.com/owner/repo
      url = url.replace(/^git@([^:]+):/, "https://$1/");
    }
    return url;
  } catch {
    return null;
  }
}

function lastTag() {
  try {
    return run("git describe --tags --abbrev=0").trim();
  } catch {
    return null;
  }
}

// conventional commit: `type(scope)!: subject`
function parseCommit(hash, subject) {
  const m = subject.match(/^(\w+)(?:\(([^)]+)\))?(!)?:\s*(.+)$/);
  if (!m) return null;
  return {
    hash,
    type: m[1],
    scope: m[2] || "",
    breaking: !!m[3],
    subject: m[4].trim(),
  };
}

function formatEntry(c, repo) {
  const scope = c.scope ? `**${c.scope}:** ` : "";
  const link = repo
    ? `([${c.hash.slice(0, 7)}](${repo}/commit/${c.hash}))`
    : `(${c.hash.slice(0, 7)})`;
  return `* ${scope}${c.subject} ${link}`;
}

function section(title, entries, repo) {
  return `### ${title}\n\n${entries.map((c) => formatEntry(c, repo)).join("\n")}`;
}

/**
 * 为 newVersion 生成一条 CHANGELOG markdown 条目。
 * 提交来自上个 tag..HEAD；仅收录 feat/fix/perf，并单独列出 BREAKING CHANGES。
 */
export function generateChangelogEntry(newVersion) {
  const repo = repoUrl();
  const prevTag = lastTag();
  const range = prevTag ? `${prevTag}..HEAD` : "HEAD";
  const log = run(`git log --no-merges --format="%H%x09%s" ${range}`).trim();

  const commits = log
    ? log
        .split("\n")
        .map((line) => {
          const idx = line.indexOf("\t");
          if (idx < 0) return null;
          return parseCommit(line.slice(0, idx), line.slice(idx + 1));
        })
        .filter(Boolean)
    : [];

  const today = new Date().toISOString().slice(0, 10);
  const compareUrl = repo
    ? prevTag
      ? `${repo}/compare/${prevTag}...v${newVersion}`
      : `${repo}/commits/v${newVersion}`
    : null;
  const header = compareUrl
    ? `## [${newVersion}](${compareUrl}) (${today})`
    : `## [${newVersion}] (${today})`;

  const grouped = {};
  const breaking = [];
  for (const c of commits) {
    if (c.breaking) breaking.push(c);
    if (TYPE_TITLES[c.type]) (grouped[c.type] ||= []).push(c);
  }

  const parts = [];
  if (breaking.length) parts.push(section("BREAKING CHANGES", breaking, repo));
  for (const type of TYPE_ORDER) {
    if (grouped[type]) parts.push(section(TYPE_TITLES[type], grouped[type], repo));
  }

  const body = parts.join("\n\n\n");
  return body ? `${header}\n\n\n${body}` : header;
}
