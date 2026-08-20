// Fetch image bytes through a real browser context (bypasses BunnyCDN's
// JS bot-challenge that blocks plain HTTP clients on some jumpthrow.pro
// media URLs).
// Usage: node fetch_via_browser.js <urls_file> <out_dir>
// urls_file: one URL per line. Writes <out_dir>/<sha1(url)>.bin plus a
// manifest.json mapping url -> {ok, path, status}.
// Requires NODE_PATH to include the system playwright install, e.g.:
//   NODE_PATH=/usr/share/nodejs node fetch_via_browser.js ...
const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
const os = require('os');
const crypto = require('crypto');

function findChrome() {
  const cacheDir = path.join(os.homedir(), '.cache/ms-playwright');
  if (!fs.existsSync(cacheDir)) return null;
  const dirs = fs.readdirSync(cacheDir).filter(d => d.startsWith('chromium-') && !d.includes('headless'));
  for (const d of dirs) {
    const candidate = path.join(cacheDir, d, 'chrome-linux64', 'chrome');
    if (fs.existsSync(candidate)) return candidate;
    const candidate2 = path.join(cacheDir, d, 'chrome-linux', 'chrome');
    if (fs.existsSync(candidate2)) return candidate2;
  }
  return null;
}

async function launchBrowser() {
  try {
    return await chromium.launch();
  } catch (e) {
    const chromePath = findChrome();
    if (!chromePath) throw e;
    return await chromium.launch({ executablePath: chromePath });
  }
}

async function main() {
  const [urlsFile, outDir] = process.argv.slice(2);
  const urls = fs.readFileSync(urlsFile, 'utf8').split('\n').map(s => s.trim()).filter(Boolean);
  fs.mkdirSync(outDir, { recursive: true });

  const browser = await launchBrowser();
  const context = await browser.newContext({
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36',
  });
  const page = await context.newPage();

  // Warm up: visit the site itself first so any shield cookie gets set.
  try {
    await page.goto('https://jumpthrow.pro/', { waitUntil: 'domcontentloaded', timeout: 20000 });
  } catch (e) {
    console.error('warmup nav failed (continuing):', e.message);
  }

  const manifest = {};
  for (const url of urls) {
    const key = crypto.createHash('sha1').update(url).digest('hex');
    try {
      const resp = await page.goto(url, { timeout: 20000 });
      const status = resp ? resp.status() : null;
      if (resp && status === 200) {
        const buf = await resp.body();
        const outPath = path.join(outDir, key + '.bin');
        fs.writeFileSync(outPath, buf);
        manifest[url] = { ok: true, path: outPath, status };
        console.log(`OK  ${status}  ${url}`);
      } else {
        manifest[url] = { ok: false, path: null, status };
        console.log(`FAIL ${status}  ${url}`);
      }
    } catch (e) {
      manifest[url] = { ok: false, path: null, status: null, error: e.message };
      console.log(`ERROR ${url} :: ${e.message}`);
    }
    await page.waitForTimeout(400);
  }

  fs.writeFileSync(path.join(outDir, 'manifest.json'), JSON.stringify(manifest, null, 2));
  try {
    await browser.close();
  } catch (e) {
    // known cleanup bug in some playwright-core versions; fetches already completed
  }
  process.exit(0);
}

main();
