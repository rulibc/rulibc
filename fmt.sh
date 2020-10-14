#!/usr/bin/env bash

cargo fmt --package rulibc --package crt0 --package crti --package crtn --package crtn ld_so --package crtn ld_so "$@"
