#!/usr/bin/env bash

set -e

BASEDIR=$(dirname "$0")/..

mkdir -p $BASEDIR/target

for i in {01..13} ; do
  if [ $i -eq 5 ]
  then
    if [ -f $BASEDIR/target/assignment05.zip ]; then
      rm $BASEDIR/target/assignment05.zip
    fi
    zip -rj $BASEDIR/target/assignment05.zip $BASEDIR/assets/why3/assignment05
    continue
  fi
  if [ -f $BASEDIR/target/assignment$i.zip ]; then
    rm $BASEDIR/target/assignment$i.zip
  fi
  zip -rj $BASEDIR/target/assignment$i.zip $BASEDIR/src/assignments/assignment$i
done
