#!/bin/sh

testCurrentVersionGreaterThanEqualToMinimum() {
  current="0.10.0"
  minMajor=0
  minMinor=9
  minPatch=1
  allowPre=false
  isGreaterThanEqualToMinimumVersion ${current} ${minMajor} ${minMinor} ${minPatch} ${allowPre}
  act=$?
  assertEquals 0 ${act}
}

testCurrentVersionLessThanMinimumMajor() {
  current="0.9.1"
  minMajor=1
  minMinor=0
  minPatch=0
  allowPre=false
  isGreaterThanEqualToMinimumVersion ${current} ${minMajor} ${minMinor} ${minPatch} ${allowPre}
  act=$?
  assertEquals 1 ${act}
}

testCurrentVersionLessThanMinimumMinor() {
  current="0.8.4"
  minMajor=0
  minMinor=9
  minPatch=0
  allowPre=false
  isGreaterThanEqualToMinimumVersion ${current} ${minMajor} ${minMinor} ${minPatch} ${allowPre}
  act=$?
  assertEquals 2 ${act}
}

# shellcheck source=src/hook_files/semver.sh
. "$(dirname "$0")"/../../src/hook_files/semver.sh
# shellcheck source=tests/hook_files/shunit2.sh
. "$(dirname "$0")"/shunit2.sh
