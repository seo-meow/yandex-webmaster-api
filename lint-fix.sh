#!/bin/bash
docker run --rm \
  -e RUN_LOCAL=true \
  -e FIX_MODE=true \
  -e FIX_GITHUB_ACTIONS_ZIZMOR=true \
  -e FIX_JSON_PRETTIER=true \
  -e FIX_MARKDOWN_PRETTIER=true \
  -e FILTER_REGEX_EXCLUDE=target/**/* \
  -e VALIDATE_RUST_2015=false \
  -e VALIDATE_BIOME_FORMAT=false \
  -e VALIDATE_BIOME_LINT=false \
  -e VALIDATE_JAVASCRIPT_ES=false \
  -e VALIDATE_JSCPD=false \
  -e VALIDATE_TYPESCRIPT_ES=false \
  -e VALIDATE_JSON=false \
  -v "$(pwd)":/tmp/lint \
  -u "$(id -u \"${USER}\")":"$(id -g \"${USER}\")" \
  ghcr.io/super-linter/super-linter:latest