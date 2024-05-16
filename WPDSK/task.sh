#!/bin/bash
if [ ! $# == 2 ]; then
  echo "Usage: $0 <old_extension> <new_extension>"
  exit 1
fi

if [ ! -d "bak/" ]; then
  mkdir bak
fi

for f in *.$1; do
  echo "Processing $f"
  cp "$f" bak/
  mv "$f" "${f%.*}.$2"
done

exit 0
