#!/bin/sh
# rusty-hook
# version {{VERSION}}

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
  echo "[rusty-hook] Upgrading rusty-hook cli..."
  echo "[rusty-hook] This may take a few seconds..."
  cargo install --force rusty-hook >/dev/null 2>&1
}

installRustyHookCli() {
  echo "[rusty-hook] Finalizing rusty-hook configuration..."
  echo "[rusty-hook] This may take a few seconds..."
  cargo install rusty-hook >/dev/null 2>&1
}

ensureMinimumRustyHookCliVersion() {
  currentVersion=$(rusty-hook -v)
  isGreaterThanEqualToMinimumVersion "${currentVersion}" ${minimumMajorCliVersion} ${minimumMinorCliVersion} ${minimumPatchCliVersion} ${allowPrereleaseCliVersion} >/dev/null 2>&1
  versionCompliance=$?
  if [ ${versionCompliance} -gt 0 ]; then
    upgradeRustyHookCli || true
  fi
}

handleRustyHookCliResult() {
  rustyHookExitCode=${1}
  hookName=${2}

  # shellcheck disable=SC2086
  if [ ${rustyHookExitCode} -eq 0 ]; then
    exit 0
  fi

  # shellcheck disable=SC2086
  if [ ${rustyHookExitCode} -eq ${noConfigFileExitCode} ]; then
    if [ "${hookName}" = "pre-commit" ]; then
      echo "[rusty-hook] rusty-hook git hooks are configured, but no config file was found"
      echo "[rusty-hook] In order to use rusty-hook, your project must have a config file"
      echo "[rusty-hook] See https://github.com/swellaby/rusty-hook#configure for more information about configuring rusty-hook"
      echo
      echo "[rusty-hook] If you were trying to remove rusty-hook, then you should also delete the git hook files to remove this warning"
      echo "[rusty-hook] See https://github.com/swellaby/rusty-hook#removing-rusty-hook for more information about removing rusty-hook from your project"
      echo
    fi
    exit 0
  else
    echo "[rusty-hook] Configured hook command failed"
    echo "[rusty-hook] ${hookName} hook rejected"
    # shellcheck disable=SC2086
    exit ${rustyHookExitCode}
  fi
}

# shellcheck source=src/hook_files/semver.sh
. "$(dirname "$0")"/semver.sh
