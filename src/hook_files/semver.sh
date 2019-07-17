#!/bin/sh
# rusty-hook
# version {{VERSION}}

isGreaterThanEqualToMinimumVersion() {
  currentVersion=${1}
  minimumMajor=${2}
  minimumMinor=${3}
  minimumPatch=${4}
  allowPrerelease=${5}

  oldIFS=${IFS}
  IFS="."
  # shellcheck disable=SC2086
  set -- ${currentVersion}
  currentMajor=${1}
  currentMinor=${2}
  suffix=${3}
  IFS="-"
  # shellcheck disable=SC2086
  set -- ${suffix}
  currentPatch=${1}
  currentPre=${2}
  IFS=${oldIFS}

  # shellcheck disable=SC2086
  if [ ${currentMajor} -lt ${minimumMajor} ]; then
    echo "major version mismatch"
    echo "minimum: ${minimumMajor}"
    echo "current: ${currentMajor}"
    return 1
  fi

  # shellcheck disable=SC2086
  if [ ${currentMinor} -lt ${minimumMinor} ]; then
    echo "minor version mismatch"
    echo "minimum: ${minimumMinor}"
    echo "current: ${currentMinor}"
    return 2
  fi

  # shellcheck disable=SC2086
  if [ ${currentPatch} -lt ${minimumPatch} ]; then
    echo "patch version mismatch"
    echo "minimum: ${minimumPatch}"
    echo "current: ${currentPatch}"
    return 3
  # shellcheck disable=SC2086
  elif [ ${currentPatch} -eq ${minimumPatch} ]; then
    if [ -z "${currentPre}" ]; then
      return 0
    elif [ "${allowPrerelease}" != "true" ]; then
      echo "pre version mismatch"
      return 4
    fi
  fi
}
