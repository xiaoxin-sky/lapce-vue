#!/usr/bin/env bash
target=(node14-macos-x64,node12-macos-x64)
pkg ./lapce-volar/main.js --output vue-language-server
# tar -zcf lapce-vue.tar.gz ./bin ./lapce-volar volt.toml
# tar -zcf lapce-volar.tar.gz ./lapce-volar
