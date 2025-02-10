#!/usr/bin/env bash

BASEDIR=$(dirname "$0")
RUSTDOC="$BASEDIR/rustdoc.sh" cargo doc $@
