#!/bin/sh

baseDir=$(dirname "$0")

echo "semver.sh tests:"
echo
"${baseDir}"/semver.sh
echo
echo "cli.sh tests:"
echo
"${baseDir}"/cli.sh
