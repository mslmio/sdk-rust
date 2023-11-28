#!/bin/bash

DIR=`dirname $0`
ROOT=$DIR/..

echo $DIR $ROOT
# Format the project.

rustfmt \
    --config-path $ROOT/.rustfmt.toml \
    $ROOT/src/*
