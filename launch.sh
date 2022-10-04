#!/usr/bin/env bash

cargo wasi build

cp -f ./target/wasm32-wasi/debug/lapce-vue.rustc.wasm  ~/Library/Application\ Support/dev.lapce.Lapce-Stable/plugins/Lapce.lapce-vue/bin/
open ~/Library/Application\ Support/dev.lapce.Lapce-Stable/logs/
cd ~/workspace/fork/lapce/target/release && ./lapce

