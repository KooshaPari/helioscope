#!/usr/bin/env python3
"""Simple HTTP server exposing health, readiness, and metrics endpoints.

Run with: python health_server.py [--port PORT]

Endpoints:
- GET /health - Health check
- GET /ready - Readiness check
- GET /live - Liveness probe
- GET /metrics - Prometheus-style metrics
"""

import argparse
import json
import sys
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse

# Add harness to path
sys.path.insert(0, "/Users/kooshapari/temp-PRODVERCEL-485/kush/heliosHarness-infra/harness/src")

from harness.health import (
    get_health_checker,
    get_metrics_collector,
    HealthChecker,
    MetricsCollector,
)


class HealthRequestHandler(BaseHTTPRequestHandler):
    """HTTP request handler for health endpoints."""
    
    def do_GET(self):
        """Handle GET requests."""
        parsed = urlparse(self.path)
        path = parsed.path
        
        if path == "/health":
            self._handle_health()
        elif path == "/ready":
            self._handle_ready()
        elif path == "/live":
            self._handle_live()
        elif path == "/metrics":
            self._handle_metrics()
        elif path == "/":
            self._handle_index()
        else:
            self._send_error(404, "Not Found")
    
    def _handle_health(self):
        """Health check endpoint."""
        checker = get_health_checker()
        result = checker.check_health()
        self._send_json(200, result.to_dict())
    
    def _handle_ready(self):
        """Readiness check endpoint."""
        checker = get_health_checker()
        result = checker.check_readiness()
        self._send_json(200, result.to_dict())
    
    def _handle_live(self):
        """Liveness probe."""
        self._send_json(200, {"status": "alive"})
    
    def _handle_metrics(self):
        """Prometheus metrics endpoint."""
        collector = get_metrics_collector()
        metrics = collector.get_metrics()
        self._send_plain(200, metrics)
    
    def _handle_index(self):
        """Root endpoint with links."""
        self._send_json(200, {
            "service": "heliosHarness",
            "endpoints": {
                "health": "/health",
                "ready": "/ready",
                "live": "/live",
                "metrics": "/metrics",
            }
        })
    
    def _send_json(self, code: int, data: dict):
        """Send JSON response."""
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())
    
    def _send_plain(self, code: int, data: str):
        """Send plain text response."""
        self.send_response(code)
        self.send_header("Content-Type", "text/plain")
        self.end_headers()
        self.wfile.write(data.encode())
    
    def _send_error(self, code: int, message: str):
        """Send error response."""
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps({"error": message}).encode())
    
    def log_message(self, format, *args):
        """Suppress default logging."""
        pass


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="HeliosHarness Health Server")
    parser.add_argument("--port", type=int, default=8080, help="Port to listen on")
    parser.add_argument("--host", default="0.0.0.0", help="Host to bind to")
    args = parser.parse_args()
    
    server = HTTPServer((args.host, args.port), HealthRequestHandler)
    print(f"Starting health server on {args.host}:{args.port}")
    print("Endpoints:")
    print("  GET /health - Health check")
    print("  GET /ready - Readiness check")
    print("  GET /live  - Liveness probe")
    print("  GET /metrics - Prometheus metrics")
    print()
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down...")
        server.shutdown()


if __name__ == "__main__":
    main()
