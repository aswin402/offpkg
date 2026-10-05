#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');
const https = require('https');
const { spawnSync } = require('child_process');

const REPO = 'https://github.com/aswin402/offpkg';
const IS_WIN = process.platform === 'win32';
const BINARY_NAME = IS_WIN ? 'offpkg.exe' : 'offpkg';
const INSTALL_DIR = path.join(os.homedir(), '.offpkg', 'bin');
const LOCAL_BIN = path.join(INSTALL_DIR, BINARY_NAME);

function getTargetAsset() {
  const platform = process.platform;
  const arch = process.arch;

  let osName = '';
  if (platform === 'linux') osName = 'linux';
  else if (platform === 'darwin') osName = 'macos';
  else if (platform === 'win32') osName = 'windows';
  else throw new Error(`Unsupported OS: ${platform}`);

  let archName = '';
  if (arch === 'x64') archName = 'x86_64';
  else if (arch === 'arm64') archName = 'aarch64';
  else throw new Error(`Unsupported architecture: ${arch}`);

  return `offpkg-${osName}-${archName}${IS_WIN ? '.exe' : ''}`;
}

function downloadBinary(url, dest) {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return downloadBinary(res.headers.location, dest).then(resolve).catch(reject);
      }
      if (res.statusCode !== 200) {
        return reject(new Error(`Failed to download binary: HTTP ${res.statusCode}`));
      }
      fs.mkdirSync(path.dirname(dest), { recursive: true });
      const file = fs.createWriteStream(dest, { mode: 0o755 });
      res.pipe(file);
      file.on('finish', () => file.close(resolve));
      file.on('error', reject);
    }).on('error', reject);
  });
}

async function main() {
  if (!fs.existsSync(LOCAL_BIN)) {
    const asset = getTargetAsset();
    const downloadUrl = `${REPO}/releases/latest/download/${asset}`;
    console.error(`[offpkg] Downloading native binary (${asset})...`);
    try {
      await downloadBinary(downloadUrl, LOCAL_BIN);
      if (!IS_WIN) fs.chmodSync(LOCAL_BIN, 0o755);
      console.error(`[offpkg] Successfully installed to ${LOCAL_BIN}`);
    } catch (err) {
      console.error(`[offpkg] Failed to download binary: ${err.message}`);
      console.error(`[offpkg] Please install offpkg using: curl -fsSL ${REPO}/raw/main/install.sh | bash`);
      process.exit(1);
    }
  }

  const result = spawnSync(LOCAL_BIN, process.argv.slice(2), { stdio: 'inherit' });
  process.exit(result.status ?? 0);
}

main().catch((err) => {
  console.error(`[offpkg error] ${err.message}`);
  process.exit(1);
});
