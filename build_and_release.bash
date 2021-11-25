#!/bin/bash
cargo build --release;
sudo cp ./target/release/rust-cli /usr/bin;