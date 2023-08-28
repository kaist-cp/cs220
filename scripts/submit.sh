#!/usr/bin/env bash

set -e

if [ $# != 1 ]
  then
    echo "==============================="
    echo "Invalid argument."
    echo "Usage: './submit.sh <assignment_number>'"
    echo "Example: './submit.sh 1' to grade assignment01"
    exit 1
fi

BASEDIR=$(dirname "$0")/..
mkdir -p $BASEDIR/target

ZIP_FILE=$(printf "$BASEDIR/target/assignment%02d.zip" $1)
SRC_PATH=$(printf "$BASEDIR/src/assignments/assignment%02d" $1)

echo $ZIP_FILE
echo $SRC_PATH
zip -rj $ZIP_FILE $SRC_PATH
