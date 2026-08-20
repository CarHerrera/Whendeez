#!/usr/bin/env python3
"""Keep nade note `image:` fields (and jumpthrow.pro extras) in sync with
their source link. Handles two sources:

  youtube/youtu.be -> image: set to the video's hqdefault thumbnail
  jumpthrow.pro  -> image: set to a locally-built position+result diagonal
                    split, the position screenshot appended into the note
                    body, and any expired <video> embed replaced with a
                    plain link to the page

Usage:
  python3 scripts/sync_nade_images.py --apply [--files path1 path2 ...]
  (omit --files to scan the whole vault; omit --apply for a dry run)

Safe to run repeatedly: already-correct/already-processed notes are skipped.
Never aborts on a single file's failure — failures are logged and skipped so
one bad network call can't block a whole batch (or a git commit).
"""
import argparse
import io
import os
import random
import re
import subprocess
import sys
import time

import requests
from PIL import Image, ImageDraw

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
IMAGES_DIR = os.path.join(REPO_ROOT, "Images")
SCRIPTS_DIR = os.path.dirname(os.path.abspath(__file__))
BROWSER_FETCH_JS = os.path.join(SCRIPTS_DIR, "fetch_via_browser.js")

UA = "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0"
TIMEOUT = 20

FM_DELIM = re.compile(r"^---\s*$")
LINK_LINE = re.compile(r'^Link:\s*"?(?P<val>.*?)"?\s*$')
IMAGE_LINE = re.compile(r'^(?P<prefix>image:[ \t]*)(?P<q>"?)(?P<url>.*?)(?P=q)[ \t]*$')
YT_ID_PATTERNS = [
    re.compile(r'youtube\.com/shorts/(?P<id>[A-Za-z0-9_-]{11})'),
    re.compile(r'youtu\.be/(?P<id>[A-Za-z0-9_-]{11})'),
    re.compile(r'[?&]v=(?P<id>[A-Za-z0-9_-]{11})'),
]
FENCED_BLOCK = re.compile(r'```.*?```', re.S)
INLINE_SETPOS = re.compile(r'^`setpos[^`]*`\s*$', re.M)


class Failure(Exception):
    pass


def log(msg):
    print(msg, file=sys.stderr)


def find_frontmatter(lines):
    if not lines or not FM_DELIM.match(lines[0]):
        return None
    for i in range(1, len(lines)):
        if FM_DELIM.match(lines[i]):
            return 1, i
    return None


def read_note(path):
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    lines = text.splitlines(keepends=True)
    fm = find_frontmatter(lines)
    if fm is None:
        return None
    start, end = fm
    link_val, img_idx, img_match = None, None, None
    for i in range(start, end):
        stripped = lines[i].rstrip("\n")
        m = LINK_LINE.match(stripped)
        if m:
            link_val = m.group("val").strip()
        m2 = IMAGE_LINE.match(stripped)
        if m2:
            img_idx, img_match = i, m2
    return {
        "path": path, "text": text, "lines": lines,
        "link": link_val, "img_idx": img_idx, "img_match": img_match,
    }


def write_image_line(note, new_value, apply):
    """Rewrite just the image: line, always normalized to 'image: ' (a bare
    'image:' with no space produces invalid YAML if the value contains a colon,
    e.g. a URL)."""
    if note["img_idx"] is None:
        raise Failure("no image: line in frontmatter")
    q = note["img_match"].group("q")
    new_line = f'image: {q}{new_value}{q}\n'
    if new_line == note["lines"][note["img_idx"]]:
        return False
    if apply:
        lines = list(note["lines"])
        lines[note["img_idx"]] = new_line
        tmp = note["path"] + ".tmp_sync"
        with open(tmp, "w", encoding="utf-8") as f:
            f.write("".join(lines))
        os.replace(tmp, note["path"])
    return True


def append_body(note, insertion, apply):
    text = note["text"]
    if insertion.strip() in text:
        return False
    m = FENCED_BLOCK.search(text)
    if m:
        new_text = text[:m.end()] + f"\n\n{insertion}\n" + text[m.end():]
    else:
        m2 = INLINE_SETPOS.search(text)
        if m2:
            new_text = text[:m2.end()] + f"\n\n{insertion}\n" + text[m2.end():]
        else:
            new_text = text.rstrip("\n") + f"\n\n{insertion}\n"
    if apply:
        tmp = note["path"] + ".tmp_sync"
        with open(tmp, "w", encoding="utf-8") as f:
            f.write(new_text)
        os.replace(tmp, note["path"])
    return True


def replace_dead_video(note, apply):
    text = open(note["path"], encoding="utf-8").read() if apply else note["text"]
    m = re.search(r'(<video[^>]*src="[^"]*"[^>]*>.*?</video>)', text, re.S)
    if not m:
        return False
    video_tag = m.group(1)
    exp_m = re.search(r'expires=(\d+)', video_tag)
    if not exp_m or int(exp_m.group(1)) >= time.time():
        return False
    if not note["link"]:
        return False
    replacement = f'[Watch lineup on jumpthrow.pro →]({note["link"]})'
    new_text = text.replace(video_tag, replacement, 1)
    if apply:
        tmp = note["path"] + ".tmp_sync"
        with open(tmp, "w", encoding="utf-8") as f:
            f.write(new_text)
        os.replace(tmp, note["path"])
    return True


# ---------- youtube ----------

def extract_video_id(link):
    for pat in YT_ID_PATTERNS:
        m = pat.search(link)
        if m:
            return m.group("id")
    return None


def sync_youtube(note, session, apply):
    video_id = extract_video_id(note["link"])
    if not video_id:
        raise Failure(f"could not parse video id from Link: {note['link']}")
    url = f"https://img.youtube.com/vi/{video_id}/hqdefault.jpg"
    try:
        r = session.get(url, timeout=TIMEOUT)
    except requests.RequestException as e:
        raise Failure(f"request error: {e}")
    if r.status_code != 200:
        raise Failure(f"HTTP {r.status_code} (bad/dead video id: {video_id})")
    stored = note["img_match"].group("url").strip() if note["img_match"] else ""
    if url == stored:
        return "already correct"
    write_image_line(note, url, apply)
    return f"updated -> {url}"


# ---------- jumpthrow.pro ----------

def fetch_via_browser(urls):
    """Fetch a list of URLs through a real browser (bypasses BunnyCDN's JS
    bot-challenge on the media CDN). Returns {url: bytes}."""
    if not urls:
        return {}
    tmp_dir = "/tmp/sync_nade_images_browser_fetch"
    os.makedirs(tmp_dir, exist_ok=True)
    urls_file = os.path.join(tmp_dir, "urls.txt")
    out_dir = os.path.join(tmp_dir, "out")
    with open(urls_file, "w") as f:
        f.write("\n".join(urls))
    env = dict(os.environ, NODE_PATH="/usr/share/nodejs")
    subprocess.run(
        ["node", BROWSER_FETCH_JS, urls_file, out_dir],
        env=env, timeout=180, capture_output=True,
    )
    import json
    manifest_path = os.path.join(out_dir, "manifest.json")
    if not os.path.exists(manifest_path):
        return {}
    manifest = json.load(open(manifest_path))
    result = {}
    for url, entry in manifest.items():
        if entry.get("ok"):
            with open(entry["path"], "rb") as f:
                result[url] = f.read()
    return result


def cover_fit(img, w, h):
    src_ratio = img.width / img.height
    dst_ratio = w / h
    if src_ratio > dst_ratio:
        new_h, new_w = h, int(h * src_ratio)
    else:
        new_w, new_h = w, int(w / src_ratio)
    resized = img.resize((new_w, new_h), Image.LANCZOS)
    left, top = (new_w - w) // 2, (new_h - h) // 2
    return resized.crop((left, top, left + w, top + h))


def make_diagonal_composite(pos_img, result_img, W=1280, H=640, slant=40, divider_px=4):
    split_x = W // 2
    panel_w = W // 2 + slant
    left_full = cover_fit(pos_img, panel_w, H)
    right_full = cover_fit(result_img, panel_w, H)

    left_mask = Image.new("L", (W, H), 0)
    ImageDraw.Draw(left_mask).polygon(
        [(0, 0), (split_x - slant // 2, 0), (split_x + slant // 2, H), (0, H)], fill=255)
    right_mask = Image.new("L", (W, H), 0)
    ImageDraw.Draw(right_mask).polygon(
        [(split_x - slant // 2, 0), (W, 0), (W, H), (split_x + slant // 2, H)], fill=255)

    canvas = Image.new("RGB", (W, H), (0, 0, 0))
    canvas.paste(left_full, (0, 0), left_mask.crop((0, 0, panel_w, H)))
    canvas.paste(right_full, (W - panel_w, 0), right_mask.crop((W - panel_w, 0, W, H)))
    ImageDraw.Draw(canvas).line(
        [(split_x - slant // 2, 0), (split_x + slant // 2, H)], fill=(0, 0, 0), width=divider_px)
    return canvas


def resolve_local_image(target, vault_root):
    if "/" in target:
        candidate = os.path.normpath(os.path.join(vault_root, target))
        if os.path.isfile(candidate):
            return candidate
    basename = os.path.basename(target)
    candidates = [basename]
    if "." not in basename:
        candidates += [basename + ext for ext in [".png", ".jpg", ".jpeg", ".webp"]]
    for root, dirs, files in os.walk(vault_root):
        if ".git" in dirs:
            dirs.remove(".git")
        for f in files:
            if f in candidates:
                return os.path.normpath(os.path.join(root, f))
    return None


def sync_jumpthrow(note, apply, browser_cache):
    stored = note["img_match"].group("url").strip() if note["img_match"] else ""
    note_name = os.path.splitext(os.path.basename(note["path"]))[0]

    # already composited by this tool -> nothing more to do for the image
    already_composited = stored.strip('"[]') == f"{note_name} Split.png"

    uuid_m = re.search(r'jumpthrow\.pro/nades/([a-f0-9-]+)', note["link"])
    if not uuid_m:
        raise Failure("could not parse nade uuid from Link")
    position_url = f"https://media.jumpthrow.pro/nades/{uuid_m.group(1)}/lineup.webp"

    actions = []

    if not already_composited:
        if stored == "":
            actions.append("skip composite (blank image:, no result source)")
        else:
            if position_url not in browser_cache:
                fetched = fetch_via_browser([position_url])
                browser_cache.update(fetched)
            if position_url not in browser_cache:
                raise Failure("could not fetch position image (jumpthrow.pro CDN blocked/unavailable)")

            if stored.startswith('[[') or stored.startswith('"[['):
                target = stored.strip('"').strip("[]")
                result_path = resolve_local_image(target, REPO_ROOT)
                if not result_path:
                    raise Failure(f"local result image not found: {target}")
                result_img = Image.open(result_path).convert("RGB")
            else:
                r = requests.get(stored, timeout=TIMEOUT)
                r.raise_for_status()
                result_img = Image.open(io.BytesIO(r.content)).convert("RGB")

            pos_img = Image.open(io.BytesIO(browser_cache[position_url])).convert("RGB")
            composite = make_diagonal_composite(pos_img, result_img)
            composite_name = f"{note_name} Split.png"
            if apply:
                os.makedirs(IMAGES_DIR, exist_ok=True)
                composite.save(os.path.join(IMAGES_DIR, composite_name), "PNG")
            write_image_line(note, f"[[{composite_name}]]", apply)
            actions.append(f"built composite -> {composite_name}")

    # append lineup screenshot into the body, if not already there
    lineup_embed_name = f"{note_name} Lineup.webp"
    if position_url not in note["text"] and f"![[{lineup_embed_name}]]" not in note["text"]:
        if position_url not in browser_cache:
            fetched = fetch_via_browser([position_url])
            browser_cache.update(fetched)
        if position_url in browser_cache:
            if apply:
                os.makedirs(IMAGES_DIR, exist_ok=True)
                with open(os.path.join(IMAGES_DIR, lineup_embed_name), "wb") as f:
                    f.write(browser_cache[position_url])
            if append_body(note, f"![[{lineup_embed_name}]]", apply):
                actions.append(f"appended lineup image -> {lineup_embed_name}")
            note = read_note(note["path"]) if apply else note  # re-read so replace_dead_video sees latest text

    if replace_dead_video(note, apply):
        actions.append("replaced dead video embed with link")

    return "; ".join(actions) if actions else "already correct"


# ---------- driver ----------

def walk_all_md(root, only_files=None):
    if only_files:
        for f in only_files:
            if f.endswith(".md") and os.path.isfile(f):
                yield f
        return
    for dirpath, dirnames, filenames in os.walk(root):
        if ".git" in dirnames:
            dirnames.remove(".git")
        if "Templates" in dirnames:
            dirnames.remove("Templates")
        for fn in filenames:
            if fn.endswith(".md"):
                yield os.path.join(dirpath, fn)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--files", nargs="*", default=None)
    args = ap.parse_args()

    session = requests.Session()
    session.headers["User-Agent"] = UA
    browser_cache = {}

    counters = {"scanned": 0, "skipped": 0, "ok": 0, "failed": 0}
    for path in walk_all_md(REPO_ROOT, args.files):
        counters["scanned"] += 1
        try:
            note = read_note(path)
        except Exception as e:
            log(f"PARSE-ERROR {path}: {e}")
            counters["failed"] += 1
            continue
        if note is None or not note["link"]:
            counters["skipped"] += 1
            continue

        link = note["link"]
        try:
            if "youtu.be" in link or "youtube.com" in link:
                result = sync_youtube(note, session, args.apply)
            elif "jumpthrow.pro" in link:
                result = sync_jumpthrow(note, args.apply, browser_cache)
            else:
                counters["skipped"] += 1
                continue
        except Failure as e:
            log(f"FAILED  {path}: {e}")
            counters["failed"] += 1
            continue
        except Exception as e:
            log(f"ERROR   {path}: {e}")
            counters["failed"] += 1
            continue

        counters["ok"] += 1
        if "already correct" not in result:
            print(f"{path}: {result}")

        time.sleep(0.3 + random.uniform(0, 0.2))

    mode = "APPLY" if args.apply else "DRY RUN"
    print(f"\n=== sync_nade_images — {mode} ===")
    print(f"scanned: {counters['scanned']}  processed: {counters['ok']}  "
          f"skipped(non-tracked): {counters['skipped']}  failed: {counters['failed']}")


if __name__ == "__main__":
    main()
