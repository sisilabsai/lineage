#!/usr/bin/env python3
"""
Simple HTTP server for the Lineage trading dashboard.
Serves the dashboard.html and app.js files with WebSocket support.

Usage:
    python serve_dashboard.py
    
Then open http://localhost:8000/dashboard.html in your browser.
"""

import http.server
import socketserver
import os
from pathlib import Path

# Get the directory where this script is located
SCRIPT_DIR = Path(__file__).parent.absolute()

# Change to the script directory
os.chdir(SCRIPT_DIR)

PORT = 8000

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add headers to prevent caching (useful for development)
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate, max-age=0')
        super().end_headers()

    def log_message(self, format, *args):
        # Custom logging
        print(f'[HTTP] {format % args}')

def main():
    print("=" * 60)
    print("🚀 Lineage Trading Dashboard HTTP Server")
    print("=" * 60)
    print(f"📁 Serving files from: {SCRIPT_DIR}")
    print(f"🌐 Open in browser: http://localhost:{PORT}/dashboard.html")
    print(f"📊 WebSocket endpoint: ws://127.0.0.1:9001")
    print("=" * 60)
    print("\n⚠️  Make sure the WebSocket server is running:")
    print("    cargo run --example ws_broadcast_v2 --release")
    print("\n✅ Starting HTTP server...")
    print(f"   Press Ctrl+C to stop\n")

    try:
        with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
            print(f"📡 Listening on http://localhost:{PORT}/")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n\n✋ Server stopped")
    except OSError as e:
        if e.errno == 48:  # Port already in use (macOS/Linux)
            print(f"❌ Port {PORT} is already in use")
            print(f"   Try a different port: python serve_dashboard.py --port 8001")
        elif e.errno == 10048:  # Port already in use (Windows)
            print(f"❌ Port {PORT} is already in use")
            print(f"   Try a different port or kill the process using this port")
        else:
            raise

if __name__ == "__main__":
    main()
