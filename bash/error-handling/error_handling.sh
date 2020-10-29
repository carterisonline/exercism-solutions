#!/usr/bin/env bash

throw_error() {
    echo "Usage: error_handling.sh <person>"
    exit 1
}

if [[ -n "${2+x}" || -z "${1+x}" ]]; then
    throw_error
else
    echo "Hello, $1"
    exit 0
fi