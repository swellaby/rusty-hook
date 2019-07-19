#!/bin/sh

# isGreaterThanEqualToMinimumVersion Tests

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

testCurrentVersionLessThanMinimumPatch() {
  current="0.8.4"
  minMajor=0
  minMinor=8
  minPatch=5
  allowPre=false
  isGreaterThanEqualToMinimumVersion ${current} ${minMajor} ${minMinor} ${minPatch} ${allowPre}
  act=$?
  assertEquals 3 ${act}
}

testCurrentVersionWithPreLessThanMinimumPatch() {
  current="0.9.2-alpha"
  minMajor=0
  minMinor=9
  minPatch=3
  allowPre=false
  isGreaterThanEqualToMinimumVersion ${current} ${minMajor} ${minMinor} ${minPatch} ${allowPre}
  act=$?
  assertEquals 3 ${act}
}

testCurrentVersionWithPrereleaseAllowed() {
  current="1.0.0-beta"
  minMajor=1
  minMinor=0
  minPatch=0
  allowPre=true
  isGreaterThanEqualToMinimumVersion ${current} ${minMajor} ${minMinor} ${minPatch} ${allowPre}
  act=$?
  assertEquals 0 ${act}
}

testCurrentVersionWithPrereleaseNotAllowed() {
  current="1.0.0-alpha"
  minMajor=1
  minMinor=0
  minPatch=0
  allowPre=false
  isGreaterThanEqualToMinimumVersion ${current} ${minMajor} ${minMinor} ${minPatch} ${allowPre}
  act=$?
  assertEquals 4 ${act}
}

testMinimumDoubleDigitVersionWithFirstDigitLessThanCurrentVersion() {
  # Current minor version is `5`, minimum minor version is `36`
  # This test ensures that the implementation is never done using char by char comparisons.
  current="1.5.0"
  minMajor=1
  minMinor=36
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
