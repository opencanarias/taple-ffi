#!/bin/bash

rustup target list --installed | grep android

rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android

rustup target list --installed | grep android
