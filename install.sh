#!/bin/bash

cargo build --release
cp ./target/release/pomodoro /usr/local/bin
