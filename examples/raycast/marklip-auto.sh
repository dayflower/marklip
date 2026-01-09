#!/bin/bash
# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title Convert clipboard Markdown <-> HTML (auto)
# @raycast.mode silent
#
# Optional parameters:
# @raycast.icon 📝
# @raycast.packageName Clipboard Utilities
#
# Documentation:
# @raycast.description Convert clipboard content Markdown <-> HTML with marklip.
# @raycast.author dayflower
# @raycast.authorURL https://github.com/dayflower/marklip

set -euo pipefail

MARKLIP_BIN="marklip"

if ! command -v "$MARKLIP_BIN" >/dev/null 2>&1; then
  echo "marklip binary not found. Install marklip and ensure the binary is on your PATH."
  exit 255
fi

if "$MARKLIP_BIN" auto --notify; then
  echo "marklip: auto conversion requested."
  exit 0
fi

status=$?
case "$status" in
  1) echo "marklip: required clipboard format is missing." ;;
  2) echo "marklip: conversion failed." ;;
  255) echo "marklip: unexpected error. Check notifications or terminal logs." ;;
  *) echo "marklip: exited with status ${status}." ;;
esac

exit "$status"
