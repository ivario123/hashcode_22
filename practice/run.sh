#!/bin/bash

set -x

for file in $(find data/ -name "*.in.txt"); do
    file_out="$(echo "${file}" | rev | cut -d '/' -f1 | rev | cut -d '.' -f1).out.txt"
    cat "${file}" | python3 practice.py > "data/${file_out}"
done