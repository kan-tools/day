#!/usr/bin/env python3
"""Render RFC 1's Markdown-with-LaTeX companion as a local reading page."""

from __future__ import annotations

import html
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "rfcs/1/denotational-semantics.md"
OUTPUT = ROOT / "rfcs/1/denotational-semantics.html"


def inline(value: str) -> str:
    escaped = html.escape(value, quote=False)
    escaped = re.sub(r"`([^`]+)`", r"<code>\1</code>", escaped)
    escaped = re.sub(
        r"\[([^\]]+)\]\(([^)]+)\)",
        r'<a href="\2">\1</a>',
        escaped,
    )
    return escaped


def render(markdown: str) -> str:
    lines = markdown.splitlines()
    body: list[str] = []
    paragraph: list[str] = []
    in_math = False
    math: list[str] = []
    in_list = False

    def flush_paragraph() -> None:
        if paragraph:
            body.append(f"<p>{inline(' '.join(paragraph))}</p>")
            paragraph.clear()

    def close_list() -> None:
        nonlocal in_list
        if in_list:
            body.append("</ul>")
            in_list = False

    for line in lines:
        if line.strip() == "$$":
            flush_paragraph()
            close_list()
            if in_math:
                body.append('<div class="math">\\[' + "\n".join(math) + "\\]</div>")
                math.clear()
            in_math = not in_math
            continue
        if in_math:
            math.append(line)
            continue
        heading = re.match(r"^(#{1,3})\s+(.+)$", line)
        if heading:
            flush_paragraph()
            close_list()
            level = len(heading.group(1))
            title = inline(heading.group(2))
            anchor = re.sub(r"[^a-z0-9]+", "-", heading.group(2).lower()).strip("-")
            body.append(f'<h{level} id="{anchor}">{title}</h{level}>')
            continue
        item = re.match(r"^-\s+(.+)$", line)
        if item:
            flush_paragraph()
            if not in_list:
                body.append("<ul>")
                in_list = True
            body.append(f"<li>{inline(item.group(1))}</li>")
            continue
        if not line.strip():
            flush_paragraph()
            close_list()
            continue
        paragraph.append(line.strip())

    flush_paragraph()
    close_list()
    return "\n".join(body)


def page(body: str) -> str:
    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="source" content="denotational-semantics.md">
  <title>Day as an indexed process equipment</title>
  <style>
    :root {{ color-scheme: light; --ink:#17211d; --muted:#5d6a64; --paper:#f6f2e8;
      --card:#fffdf7; --rule:#c9d2c7; --accent:#285f50; --code:#e8eee8; }}
    * {{ box-sizing:border-box; }}
    body {{ margin:0; color:var(--ink); background:var(--paper); font:18px/1.66 Georgia,serif; }}
    header {{ padding:4.5rem max(1.5rem,calc((100vw - 760px)/2)); background:#183d34; color:#f9f5e9; }}
    header p {{ max-width:720px; color:#d6e5dd; margin:.65rem 0 0; }}
    main {{ max-width:820px; margin:2.5rem auto 6rem; padding:0 2rem 4rem; background:var(--card);
      box-shadow:0 18px 50px rgba(34,46,39,.09); border-top:5px solid var(--accent); }}
    h1,h2,h3 {{ font-family:ui-sans-serif,system-ui,sans-serif; line-height:1.16; letter-spacing:-.025em; }}
    main h1 {{ display:none; }} h2 {{ margin:3.4rem 0 1rem; font-size:1.55rem; }}
    h3 {{ margin-top:2rem; }} p,li {{ max-width:72ch; }}
    a {{ color:var(--accent); text-underline-offset:3px; }}
    code {{ font: .9em ui-monospace,SFMono-Regular,monospace; background:var(--code); padding:.12em .3em; border-radius:4px; }}
    .math {{ overflow-x:auto; margin:1.5rem 0; padding:1rem; border-left:3px solid var(--rule); }}
    .source {{ font:14px/1.4 ui-sans-serif,system-ui,sans-serif; color:var(--muted); margin-top:1.25rem; }}
    @media (max-width:650px) {{ body {{ font-size:16px; }} header {{ padding-top:3rem; }} main {{ margin:0; padding:0 1.25rem 3rem; }} }}
  </style>
  <script>window.MathJax={{tex:{{inlineMath:[['$','$'],['\\(','\\)']] }},svg:{{fontCache:'global'}}}};</script>
  <script defer src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js"></script>
</head>
<body>
  <header>
    <h1>Day as an indexed process equipment</h1>
    <p>A denotational companion to Draft RFC 1: frames, predicates, processes, bridges, and witness-bearing realization cells.</p>
    <p class="source">Canonical source: <a href="denotational-semantics.md">denotational-semantics.md</a></p>
  </header>
  <main>
{body}
  </main>
</body>
</html>
"""


def main() -> None:
    OUTPUT.write_text(page(render(SOURCE.read_text())), encoding="utf-8")


if __name__ == "__main__":
    main()
