#!/usr/bin/env bash

set -e

BASEDIR=$(dirname "$0")/..

mkdir -p $BASEDIR/target

zip -rj $BASEDIR/target/assignment04.zip src/assignments/assignment04
zip -rj $BASEDIR/target/assignment11.zip src/assignments/assignment11
