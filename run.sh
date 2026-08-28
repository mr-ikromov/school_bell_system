#!/usr/bin/env bash
set -e
DIR="$(cd "$(dirname "$0")" && pwd)"
PORT=8777
URL="http://127.0.0.1:$PORT/index.html"

if ! curl -s -o /dev/null "$URL"; then
  (setsid nohup python3 "$DIR/dev-server.py" $PORT >/tmp/bell-server.log 2>&1 </dev/null &)
  for i in $(seq 1 20); do curl -s -o /dev/null "$URL" && break; sleep .2; done
fi

BROWSER=$(command -v google-chrome || command -v chromium || command -v chromium-browser)
if [ -n "$BROWSER" ]; then
  nohup "$BROWSER" --app="$URL" \
      --window-size=800,580 --window-position=120,80 \
      --user-data-dir=/tmp/bell-preview-profile \
      >/dev/null 2>&1 &
else
  xdg-open "$URL"
fi

echo "Ochildi: $URL"
echo "To'xtatish: pkill -f dev-server.py"
