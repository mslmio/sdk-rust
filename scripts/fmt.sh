#!/bin/bash

DIR=`dirname $0`
ROOT=$DIR/..

rustfmt \
    --config-path $ROOT/.rustfmt.toml \
    $ROOT/src/*
