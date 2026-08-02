import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { createInterface } from "node:readline";
import { generateChangelogEntry } from "./changelog.mjs";
import { execSync } from "node:child_process";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, "..");

const FILES = {
  "package.json": {
    path: resolve(root, "package.json"),
    read: (content) => JSON.parse(content).version,
    write: (content, version) =>
      content.replace(
        /"version"\s*:\s*"[^"]+"/,
        `"version": "${version}"`
      ),
  },
  "src-tauri/tauri.conf.json": {
    path: resolve(root, "src-tauri/tauri.conf.json"),
    read: (content) => JSON.parse(content).version,
    write: (content, version) =>
      content.replace(
        /"version"\s*:\s*"[^"]+"/,
        `"version": "${version}"`
      ),
  },
  "src-tauri/Cargo.toml": {
    path: resolve(root, "src-tauri/Cargo.toml"),
    read: (content) => {
      const match = content.match(/^version\s*=\s*"([^"]+)"/m);
      return match ? match[1] : null;
    },
    write: (content, version) =>
      content.replace(
        /^(version\s*=\s*)"[^"]+"/m,
        `$1"${version}"`
      ),
  },
};

function bump(current, type) {
  const parts = current.split(".").map(Number);
  if (type === "patch") return `${parts[0]}.${parts[1]}.${parts[2] + 1}`;
  if (type === "minor") return `${parts[0]}.${parts[1] + 1}.0`;
  if (type === "major") return `${parts[0] + 1}.0.0`;
  return null;
}

function run(cmd) {
  console.log(`  > ${cmd}`);
  execSync(cmd, { stdio: "inherit", cwd: root });
}

async function question(rl, prompt) {
  return new Promise((resolve) => rl.question(prompt, resolve));
}

async function main() {
  const pkgPath = FILES["package.json"].path;
  const current = FILES["package.json"].read(readFileSync(pkgPath, "utf-8"));

  console.log(`\n  Current version: ${current}\n`);

  const patch = bump(current, "patch");
  const minor = bump(current, "minor");
  const major = bump(current, "major");

  console.log("  Select new version:");
  console.log(`  1) ${patch} (patch)`);
  console.log(`  2) ${minor} (minor)`);
  console.log(`  3) ${major} (major)`);
  console.log(`  4) Custom`);
  console.log();

  const rl = createInterface({ input: process.stdin, output: process.stdout });
  const choice = await question(rl, "  Choose (1-4): ");

  let newVersion;
  if (choice === "1") newVersion = patch;
  else if (choice === "2") newVersion = minor;
  else if (choice === "3") newVersion = major;
  else if (choice === "4") {
    newVersion = await question(rl, "  Enter version: ");
    if (!/^\d+\.\d+\.\d+$/.test(newVersion)) {
      console.error(`  Invalid version: ${newVersion}`);
      rl.close();
      process.exit(1);
    }
  } else {
    console.error("  Invalid choice");
    rl.close();
    process.exit(1);
  }

  rl.close();

  if (newVersion === current) {
    console.error(`  Version unchanged (${current})`);
    process.exit(1);
  }

  console.log(`\n  Bumping ${current} → ${newVersion}\n`);

  for (const [name, file] of Object.entries(FILES)) {
    const content = readFileSync(file.path, "utf-8");
    const existing = file.read(content);
    if (existing !== current) {
      console.error(`  ${name} has version ${existing}, expected ${current}. Aborting.`);
      process.exit(1);
    }
    writeFileSync(file.path, file.write(content, newVersion));
    console.log(`  ✓ ${name}`);
  }

  const tag = `v${newVersion}`;

  // 自动生成 CHANGELOG 条目（若该版本尚未手动维护）
  const changelogPath = resolve(root, "CHANGELOG.md");
  if (existsSync(changelogPath)) {
    const prev = readFileSync(changelogPath, "utf-8");
    if (prev.includes(`## [${newVersion}]`)) {
      console.log(`  • CHANGELOG.md 已含 ${newVersion} 条目，跳过生成`);
    } else {
      const entry = generateChangelogEntry(newVersion);
      writeFileSync(
        changelogPath,
        prev.replace(/^# Changelog\n+/, () => `# Changelog\n\n${entry}\n\n`)
      );
      console.log(`  ✓ CHANGELOG.md`);
    }
  }

  console.log(`\n  Committing and tagging...\n`);
  run(`git add ${Object.values(FILES).map((f) => f.path).join(" ")} ${changelogPath}`);
  run(`git commit -m "chore: release ${tag}"`);
  run(`git tag ${tag}`);

  console.log(`\n  Pushing to origin...\n`);
  run(`git push origin main`);
  run(`git push origin ${tag}`);
  run(`git push origin main:release --force`);

  console.log(`\n  ✓ Released ${tag}`);
  console.log(`  CI will build and upload to GitHub Release.\n`);
}

main().catch((e) => {
  console.error(e.message);
  process.exit(1);
});
