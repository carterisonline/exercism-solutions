#!/usr/bin/env bash

if [[ -z "${1+x}" || -z "${2+x}" ]]; then
    echo "Usage: hamming.sh <string1> <string2>"
    exit 1
elif [[ ${#1} != ${#2} ]]; then
    echo "left and right strands must be of equal length"
    exit 1
fi

i=0
for (( c=0; c<${#1}; c++ )); do
    if [[ "${1:$c:1}" != "${2:$c:1}" ]]; then
        ((i=i+1))
    fi
done

echo $i