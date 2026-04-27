# Implementation

The workflow now:

- uses `actions/checkout@v4` with `persist-credentials: false`
- downloads ZAP from `releases/latest/download/zap-full-linux-amd64.tar.gz`

This removes the pinned dead asset URL that caused the current main failure.
