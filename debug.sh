#!/bin/sh
cargo build; cp ./target/debug/cargo-gen ~/.cargo/bin/cargo-gen; cargo gen tests/files/index.html;
