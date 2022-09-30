#!/usr/bin/env bash

cargo wasi build

cp -f "/Users/skymac/workplace/lapce-vue/target/wasm32-wasi/debug/lapce-vue.rustc.wasm"  "/Users/skymac/Library/Application Support/dev.lapce.Lapce-Stable/plugins/Lapce.lapce-vue/bin/"

cd /Users/skymac/workplace/fork/lapce/target/release/ && ./lapce
