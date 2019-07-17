#!/bin/sh
# rusty-hook
# version {{VERSION}}

hookName=$(basename "$0")
gitParams="$*"
# shellcheck disable=SC2170,SC1083
minimumMajorCliVersion={{MINIMUM_MAJOR}}
# shellcheck disable=SC2170,SC1083
minimumMinorCliVersion={{MINIMUM_MINOR}}
# shellcheck disable=SC2170,SC1083
minimumPatchCliVersion={{MINIMUM_PATCH}}
# shellcheck disable=SC2170,SC1083
allowPrereleaseCliVersion={{MINIMUM_ALLOW_PRE}}

upgradeRustyHookCli() {
  echo "Upgrading rusty-hook cli..."
  echo "This may take a few seconds..."
  cargo install --force rusty-hook >/dev/null 2>&1
}

ensureMinimumRustyHookCliVersion() {
  currentVersion=$(rusty-hook -v)
  oldIFS=$IFS
  IFS="."
  # shellcheck disable=SC2086
  set -- $currentVersion
  currentMajor=$1
  currentMinor=$2
  suffix=$3
  IFS="-"
  set -- $suffix
  currentPatch=$1
  currentPre=$2
  IFS=$oldIFS

  # shellcheck disable=SC2086
  if [ $currentMajor -lt $minimumMajorCliVersion ]; then
    echo "major version mismatch"
    upgradeRustyHookCli
    return
  fi

  # shellcheck disable=SC2086
  if [ $currentMinor -lt $minimumMinorCliVersion ]; then
    echo "major version mismatch"
    upgradeRustyHookCli
    return
  fi

  # shellcheck disable=SC2086
  if [ $currentPatch -lt $minimumPatchCliVersion ]; then
    echo "patch version mismatch"
    upgradeRustyHookCli
    return
  fi

  if [ -z "$currentPre" ]; then
    return
  elif [ "$allowPrereleaseCliVersion" != "true" ]; then
    echo "pre version mismatch"
    upgradeRustyHookCli
  fi
}

if ! command -v rusty-hook >/dev/null 2>&1; then
  if [ -z "${RUSTY_HOOK_SKIP_AUTO_INSTALL}" ]; then
    echo "Finalizing rusty-hook configuration..."
    echo "This may take a few seconds..."
    cargo install rusty-hook >/dev/null 2>&1
  else
    echo "rusty-hook is not installed, and auto install is disabled"
    echo "skipping $hookName hook"
    echo "You can reinstall it using 'cargo install rusty-hook' or delete this hook"
    exit 0
  fi
else
  ensureMinimumRustyHookCliVersion || true
fi

rusty-hook run --hook "$hookName" "$gitParams"
rhExitCode=$?

if [ $rhExitCode -ne 0 ]; then
# shellcheck disable=SC2170,SC1083
  if [ $rhExitCode -eq {{NO_CONFIG_FILE_EXIT_CODE}} ]; then
    if [ "$hookName" = "pre-commit" ]; then
      echo "rusty-hook git hooks are configured, but no config file was found"
      echo "In order to use rusty-hook, your project must have a config file"
      echo "See https://github.com/swellaby/rusty-hook#configure for more information"
      echo
      echo "If you were trying to remove rusty-hook, then you should also delete the git hook files to remove this warning"
      echo
    fi
    exit 0
  else
    echo "Configured hook command failed"
    echo "$hookName hook rejected"
    exit $rhExitCode
  fi
fi
