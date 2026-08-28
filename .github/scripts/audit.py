#!/usr/bin/env python3
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
html = (ROOT / "src/index.html").read_text()
jsf = {p.name: p.read_text() for p in sorted((ROOT / "src/js").glob("*.js"))}
cssf = {p.name: p.read_text() for p in sorted((ROOT / "src/css").glob("*.css"))}
js, css = "\n".join(jsf.values()), "\n".join(cssf.values())
rust = "\n".join(p.read_text() for p in (ROOT / "src-tauri/src").glob("*.rs"))

xato = []

def tekshir(nom, royxat):
    print(f"  {nom:26} {royxat or 'toza'}")
    if royxat:
        xato.append(nom)
sels = set()
for t in cssf.values():
    tt = re.sub(r"/\*.*?\*/", "", t, flags=re.S)
    sels |= {m.group(2) for m in re.finditer(r"(^|[\s,>+~(])\.([A-Za-z][\w-]*)", tt, re.M)}
tekshir("o'lik CSS klass", [c for c in sorted(sels)
                           if not re.search(rf'''["'\s.]{re.escape(c)}\b''', html + js)])
tekshir("o'lik CSS token", [v for v in re.findall(r"^\s*(--[\w-]+):", cssf["theme.css"], re.M)
                           if not re.search(rf"var\({re.escape(v)}[,)]", css)])
ids = re.findall(r'id="([\w-]+)"', html)
tekshir("o'lik HTML ID", [i for i in ids
                         if not re.search(rf'''[#'"]{re.escape(i)}\b''', js + css)
                         and f'"{i}"' not in html.replace(f'id="{i}"', "")])
tekshir("takroriy HTML ID", [i for i in set(ids) if ids.count(i) > 1])
olik = []
for f, t in jsf.items():
    for m in re.finditer(r"^(?:export\s+)?(?:async\s+)?(?:function|const|let|class)\s+([A-Za-z_$][\w$]*)", t, re.M):
        n = m.group(1)
        pat = rf"(?<![\w$]){re.escape(n)}(?![\w$])"
        seen = (len(re.findall(pat, t)) - 1
                + sum(len(re.findall(pat, o)) for k, o in jsf.items() if k != f)
                + len(re.findall(pat, html)))
        if seen == 0:
            olik.append(f"{f}::{n}")
tekshir("o'lik JS e'lon", olik)
K = r"[\w.-]+"
src = jsf["i18n.js"]
i0 = src.index("const DICT")
sets = {}
for lang in ("uz", "en", "ru"):
    m = re.search(rf"^  {lang}:\s*\{{", src[i0:], re.M)
    i, d = i0 + m.end(), 1
    while d:
        d += (src[i] == "{") - (src[i] == "}")
        i += 1
    sets[lang] = set(re.findall(rf"'({K})'\s*:", src[i0 + m.end():i]))

tekshir("tillar farqi", sorted(sets["uz"] ^ sets["en"]) + sorted(sets["uz"] ^ sets["ru"]))

used = (set(re.findall(rf'data-i18n(?:-title)?="({K})"', html))
        | set(re.findall(rf"\bt\('({K})'", js))
        | {f"err.{x}" for x in re.findall(r"tErr\('([\w-]+)'", js)})
kodlar = {f"err.{c}" for c in re.findall(r'(?:Err\(|ok_or\(|map_err\(\|_\| )"([a-z][a-z0-9-]+)"', rust)}
tekshir("yetishmayotgan kalit", sorted((used | kodlar) - sets["uz"]))
tekshir("ortiqcha kalit", sorted(sets["uz"] - used - kodlar))
tekshir("nomutanosib qavs", [f for f, t in cssf.items() if t.count("{") != t.count("}")])

if xato:
    print(f"\nAUDIT YIQILDI: {', '.join(xato)}")
    sys.exit(1)
print("\nAudit toza.")
