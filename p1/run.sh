#!/bin/bash

set -x

for file in $(find . -name "*.in"); do
    name="$(basename $file | cut -d '.' -f1).out"
    cat $file | python3 p1.py > $name    
done
