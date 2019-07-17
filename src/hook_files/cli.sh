#!/bin/sh
# rusty-hook
# version {{VERSION}}

# shellcheck source=src/hook_files/semver.sh
. "$(dirname "$0")"/semver.sh

# shellcheck disable=SC2170,SC1083
minimumMajorCliVersion={{MINIMUM_MAJOR}}
# shellcheck disable=SC2170,SC1083
minimumMinorCliVersion={{MINIMUM_MINOR}}
# shellcheck disable=SC2170,SC1083
minimumPatchCliVersion={{MINIMUM_PATCH}}
# shellcheck disable=SC2170,SC1083
allowPrereleaseCliVersion={{MINIMUM_ALLOW_PRE}}
# shellcheck disable=SC2170,SC1083
noConfigFileExitCode={{NO_CONFIG_FILE_EXIT_CODE}}

upgradeRustyHookCli() {
  echo "Upgrading rusty-hook cli..."
  echo "This may take a few seconds..."
  cargo install --force rusty-hook >/dev/null 2>&1
}

installRustyHookCli() {
  echo "Finalizing rusty-hook configuration..."
  echo "This may take a few seconds..."
  cargo install rusty-hook >/dev/null 2>&1
}

ensureMinimumRustyHookCliVersion() {
  currentVersion=$(rusty-hook -v)
  # currentVersion="1.0.0-alpha"
  isGreaterThanEqualToMinimumVersion "${currentVersion}" ${minimumMajorCliVersion} ${minimumMinorCliVersion} ${minimumPatchCliVersion} ${allowPrereleaseCliVersion} >/dev/null 2>&1
  versionCompliance=$?
  if [ ${versionCompliance} -gt 0 ]; then
    # echo "version compliance: ${versionCompliance}"
    upgradeRustyHookCli || true
  # else
  #   echo "minimum version set!"
  fi
}

handleRustyHookCliResult() {
  rustyHookExitCode=${1}
  hookName=${2}

  # shellcheck disable=SC2086
  if [ ${rustyHookExitCode} -ne 0 ]; then
    # shellcheck disable=SC2086
    if [ ${rustyHookExitCode} -eq ${noConfigFileExitCode} ]; then
      if [ "${hookName}" = "pre-commit" ]; then
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
      echo "${hookName} hook rejected"
      # shellcheck disable=SC2086
      exit ${rustyHookExitCode}
    fi
  fi
}
