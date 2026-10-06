#!/usr/bin/env bash
# qs looks things up from the terminal.
# Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

set -euo pipefail

if (($# == 0)); then
    tmpfile=$(mktemp)
    trap 'rm -f "$tmpfile"' EXIT
    ${EDITOR:-vim} "$tmpfile"
    if [[ ! -s "$tmpfile" ]]; then
        echo "quicksearch: empty prompt" >&2
        exit 1
    fi
    mapfile -t qs_lines < "$tmpfile"
    set -- "${qs_lines[*]}"
fi

python3 - "$@" <<'PY'
import html
import json
import re
import sys
import urllib.parse
import urllib.request
import urllib.error

RESET = "\033[0m"
BOLD = "\033[1m"
DIM = "\033[2m"
ITALIC = "\033[3m"
UNDERLINE = "\033[4m"
OSC8_START = "\033]8;;"
OSC8_END = "\033\\"
FG_BLUE = "\033[38;5;75m"
FG_CYAN = "\033[38;5;81m"
FG_GREEN = "\033[38;5;114m"
FG_YELLOW = "\033[38;5;221m"
FG_ORANGE = "\033[38;5;215m"
FG_MAGENTA = "\033[38;5;213m"
FG_RED = "\033[38;5;203m"
FG_GRAY = "\033[38;5;246m"
BG_CODE = "\033[48;5;236m"
TIMEOUT = 5

q = " ".join(sys.argv[1:]).strip()
headers = {"User-Agent": "Mozilla/5.0"}
query_terms = sorted(
    {term.lower() for term in re.findall(r"[A-Za-z0-9_./:-]+", q) if len(term) > 1},
    key=len,
    reverse=True,
)


def style(text, *codes):
    return "".join(codes) + text + RESET


def hyperlink(url, label=None):
    label = url if label is None else label
    return f"{OSC8_START}{url}{OSC8_END}{label}{OSC8_START}{OSC8_END}"


ANSI_RE = re.compile(r"\033\[[0-9;]*m")
RESULT_LINK_RE = re.compile(
    r"<a[^>]*href=['\"]([^'\"]+)['\"][^>]*class=['\"]result-link['\"][^>]*>(.*?)</a>"
    r"|<a[^>]*class=['\"]result-link['\"][^>]*href=['\"]([^'\"]+)['\"][^>]*>(.*?)</a>",
    re.S,
)
RESULT_SNIPPET_RE = re.compile(r"<td class=['\"]result-snippet['\"]>(.*?)</td>", re.S)
DDG_DJS_RE = re.compile(r"https://links\.duckduckgo\.com/d\.js[^\"']+")


def underline_terms(text, preserve_ansi=False):
    if not query_terms or not text:
        return text

    if preserve_ansi:
        visible = []
        raw_indexes = []
        states = []
        active = ""
        i = 0
        while i < len(text):
            m = ANSI_RE.match(text, i)
            if m:
                seq = m.group(0)
                active = "" if seq == RESET else seq
                i = m.end()
                continue
            visible.append(text[i])
            raw_indexes.append(i)
            states.append(active)
            i += 1
        plain = "".join(visible)
        if not plain:
            return text
        ranges = _match_ranges(plain)
        if not ranges:
            return text
        parts = []
        cursor = 0
        for start, end in ranges:
            raw_start = raw_indexes[start]
            raw_end = raw_indexes[end - 1] + 1
            restore = states[end] if end < len(states) else ""
            parts.append(text[cursor:raw_start])
            parts.append(UNDERLINE + FG_YELLOW + text[raw_start:raw_end] + RESET + restore)
            cursor = raw_end
        parts.append(text[cursor:])
        return "".join(parts)

    ranges = _match_ranges(text)
    if not ranges:
        return text
    parts = []
    cursor = 0
    for start, end in ranges:
        parts.append(text[cursor:start])
        parts.append(style(text[start:end], UNDERLINE, FG_YELLOW))
        cursor = end
    parts.append(text[cursor:])
    return "".join(parts)


def _match_ranges(text):
    ranges = []
    lower = text.lower()
    for term in query_terms:
        for match in re.finditer(re.escape(term), lower):
            start, end = match.span()
            if start > 0 and text[start - 1].isalnum():
                continue
            if end < len(text) and text[end:end + 1].isalnum():
                continue
            ranges.append((start, end))
    if not ranges:
        return []
    ranges.sort()
    merged = []
    for start, end in ranges:
        if not merged or start > merged[-1][1]:
            merged.append([start, end])
        else:
            merged[-1][1] = max(merged[-1][1], end)
    return [(start, end) for start, end in merged]


BASH_WORDS = {
    "if", "then", "else", "elif", "fi", "for", "while", "do", "done", "case", "esac",
    "in", "function", "select", "until", "export", "local", "readonly", "return",
}
GIT_WORDS = {
    "git", "commit", "checkout", "switch", "rebase", "merge", "cherry-pick", "fetch",
    "pull", "push", "status", "log", "diff", "stash", "restore", "branch", "tag",
    "reset", "add", "rm", "mv", "show", "blame", "bisect",
}
PY_WORDS = {
    "def", "class", "import", "from", "return", "if", "elif", "else", "for", "while",
    "try", "except", "finally", "with", "as", "pass", "break", "continue", "yield", "lambda",
    "True", "False", "None",
}
JS_WORDS = {
    "function", "const", "let", "var", "return", "if", "else", "for", "while", "import",
    "from", "export", "class", "new", "await", "async", "try", "catch", "finally",
}


def colorize_code_line(text, lang=""):
    s = text
    s = re.sub(r"(#.*)$", lambda m: style(m.group(1), FG_GRAY), s)
    s = re.sub(r"(//.*)$", lambda m: style(m.group(1), FG_GRAY), s)
    s = re.sub(r"\b(-{1,2}[A-Za-z0-9][A-Za-z0-9-]*)\b", lambda m: style(m.group(1), FG_CYAN), s)
    s = re.sub(r"\b([A-Z_][A-Z0-9_]*)=", lambda m: style(m.group(1), FG_ORANGE) + "=", s)
    s = re.sub(r"([\"'])(.*?)(\1)", lambda m: style(m.group(0), FG_GREEN), s)
    s = re.sub(r"\b(\d+(?:\.\d+)?)\b", lambda m: style(m.group(1), FG_MAGENTA), s)

    keywords = set()
    lang = lang.lower()
    if lang in {"bash", "sh", "shell", "zsh"}:
        keywords = BASH_WORDS | GIT_WORDS
    elif lang in {"git", "diff", "patch"}:
        keywords = GIT_WORDS
    elif lang in {"python", "py"}:
        keywords = PY_WORDS
    elif lang in {"javascript", "js", "typescript", "ts", "tsx", "jsx"}:
        keywords = JS_WORDS
    elif lang in {"markdown", "md"}:
        keywords = {"TODO", "NOTE", "TIP"}
    else:
        keywords = GIT_WORDS | BASH_WORDS

    if keywords:
        pattern = r"\b(" + "|".join(re.escape(word) for word in sorted(keywords, key=len, reverse=True)) + r")\b"
        s = re.sub(pattern, lambda m: style(m.group(1), BOLD, FG_BLUE), s)

    s = re.sub(r"\b(/[\w./-]+)\b", lambda m: style(m.group(1), FG_YELLOW), s)
    return underline_terms(s, preserve_ansi=True)


INLINE_CODE_RE = re.compile(r"`([^`]+)`")
BOLD_RE = re.compile(r"\*\*(.+?)\*\*|__(.+?)__")
ITALIC_RE = re.compile(r"(?<!\*)\*(?!\s)(.+?)(?<!\s)\*(?!\*)|(?<!_)_(?!\s)(.+?)(?<!\s)_(?!_)")
LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")


def strip_tags(text):
    return re.sub(r"<[^>]+>", "", text)


def fetch_text(url, extra_headers=None):
    request_headers = headers if not extra_headers else {**headers, **extra_headers}
    return urllib.request.urlopen(
        urllib.request.Request(url, headers=request_headers), timeout=TIMEOUT
    ).read().decode("utf-8", "ignore")


def fallback_results(query, limit=3):
    lite_url = "https://lite.duckduckgo.com/lite/?q=" + urllib.parse.quote(query)
    lite_html = fetch_text(lite_url)

    matches = list(RESULT_LINK_RE.finditer(lite_html))
    results = []
    for match in matches:
        url = html.unescape(match.group(1) or match.group(3) or "")
        title = html.unescape(strip_tags(match.group(2) or match.group(4) or "").strip())
        if url.startswith("//"):
            url = "https:" + url
        if "duckduckgo.com/l/?" in url:
            parsed = urllib.parse.urlparse(url)
            target = urllib.parse.parse_qs(parsed.query).get("uddg", [""])[0]
            if target:
                url = urllib.parse.unquote(target)
        tail = lite_html[match.end():match.end() + 1200]
        snippet_match = RESULT_SNIPPET_RE.search(tail)
        snippet = ""
        if snippet_match:
            snippet = html.unescape(strip_tags(snippet_match.group(1)).strip())
            snippet = re.sub(r"\s+", " ", snippet)
        if title and url:
            results.append({"title": title, "url": url, "snippet": snippet})
        if len(results) >= limit:
            break
    return lite_url, results


def format_markdown_line(line):
    if not line.strip():
        return line
    if line.startswith(("> ", ">> ")):
        return style(underline_terms(line), FG_GRAY)
    if re.match(r"^#{1,6}\s", line):
        return style(underline_terms(line), BOLD, FG_CYAN)
    if re.match(r"^\s*[-*+]\s", line):
        bullet = re.match(r"^(\s*[-*+]\s)(.*)$", line)
        return style(bullet.group(1), FG_ORANGE) + underline_terms(bullet.group(2))
    if re.match(r"^\s*\d+[.)]\s", line):
        item = re.match(r"^(\s*\d+[.)]\s)(.*)$", line)
        return style(item.group(1), FG_ORANGE) + underline_terms(item.group(2))
    if re.match(r"^\s*```", line):
        return style(line, BOLD, FG_MAGENTA)

    s = line
    s = LINK_RE.sub(lambda m: style(m.group(1), UNDERLINE, FG_BLUE) + style(f" ({m.group(2)})", FG_GRAY), s)
    s = INLINE_CODE_RE.sub(lambda m: style(f" {m.group(1)} ", BG_CODE, FG_GREEN), s)
    s = BOLD_RE.sub(lambda m: style(m.group(1) or m.group(2), BOLD), s)
    s = ITALIC_RE.sub(lambda m: style(m.group(1) or m.group(2), ITALIC), s)
    return underline_terms(s)


def print_results(search_url, results):
    print(style("Search:", BOLD) + " " + underline_terms(search_url))
    print()
    print(style("Top results:", BOLD, FG_CYAN))
    for i, result in enumerate(results, 1):
        print(style(f"{i}. ", FG_ORANGE) + underline_terms(result["title"]))
        print("   " + hyperlink(result["url"], style(result["url"], FG_BLUE, UNDERLINE)))
        if result["snippet"]:
            print("   " + underline_terms(result["snippet"]))
        print()
    print(style("Tip:", BOLD, FG_GRAY) + " in foot, press Ctrl+Shift+o to open links")
    sys.stdout.flush()


def fetch_summary_payload(query):
    search_url = "https://duckduckgo.com/?q=" + urllib.parse.quote(query)
    search_html = fetch_text(search_url)
    match = DDG_DJS_RE.search(search_html)
    if not match:
        return None
    djs = fetch_text(
        html.unescape(match.group(0)),
        extra_headers={"Referer": "https://duckduckgo.com/"},
    )
    marker = "DDG.deep.deepPayload = "
    start = djs.find(marker)
    if start < 0:
        return None
    payload, _ = json.JSONDecoder().raw_decode(djs[start + len(marker):])
    return payload


def print_answer(answer):
    in_code = False
    code_lang = ""
    for raw_line in answer.splitlines():
        fence = re.match(r"^\s*```\s*([A-Za-z0-9_+-]*)\s*$", raw_line)
        if fence:
            code_lang = fence.group(1)
            print(style(raw_line, BOLD, FG_MAGENTA))
            in_code = not in_code
            if not in_code:
                code_lang = ""
            continue
        if in_code:
            print(colorize_code_line(raw_line, code_lang))
        else:
            print(format_markdown_line(raw_line))


def print_summary_if_available(query):
    try:
        payload = fetch_summary_payload(query)
    except (OSError, ValueError, urllib.error.URLError):
        return False
    if not payload:
        return False
    for ia in payload.get("instantAnswers", []):
        data = ia.get("data", {})
        answer = data.get("answer")
        if data.get("action") == "answer" and answer:
            print()
            print(style("AI summary:", BOLD, FG_CYAN))
            print_answer(answer)
            return True
    return False


try:
    fallback_url, results = fallback_results(q)
except (OSError, ValueError, urllib.error.URLError):
    fallback_url, results = "", []

if results:
    print_results(fallback_url, results)
    print_summary_if_available(q)
    raise SystemExit(0)

if print_summary_if_available(q):
    raise SystemExit(0)

if fallback_url:
    print(style("No parsed results found.", BOLD, FG_ORANGE))
    print(style("Search:", BOLD) + " " + underline_terms(fallback_url))
    raise SystemExit(1)

raise SystemExit("quicksearch: no search results returned for this query")
PY
