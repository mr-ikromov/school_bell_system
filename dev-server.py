#!/usr/bin/env python3
"""Keshsiz static server — brauzer eski CSS/JS'ni ushlab qolmasligi uchun."""
import functools, http.server, socketserver, sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8777

class H(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cache-Control', 'no-store, must-revalidate')
        self.send_header('Pragma', 'no-cache')
        self.send_header('Expires', '0')
        super().end_headers()
    def log_message(self, *a): pass

socketserver.TCPServer.allow_reuse_address = True
with socketserver.TCPServer(('127.0.0.1', PORT),
        functools.partial(H, directory='src')) as httpd:
    httpd.serve_forever()
