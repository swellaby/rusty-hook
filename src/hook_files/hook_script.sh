#!/bin/sh
# rusty-hook
# version {{VERSION}}

hookName=$(basename "$0")
gitParams="$*"

# shellcheck source=src/hook_files/cli.sh
. "$(dirname "$0")"/cli.sh

if ! command -v rusty-hook >/dev/null 2>&1; then
  if [ -z "${RUSTY_HOOK_SKIP_AUTO_INSTALL}" ]; then
    installRustyHookCli
  else
    echo "[rusty-hook] rusty-hook is not installed, and auto install is disabled"
    echo "[rusty-hook] skipping ${hookName} hook"
    echo "[rusty-hook] You can reinstall it using 'cargo install rusty-hook' or delete this hook"
    exit 0
  fi
else
  ensureMinimumRustyHookCliVersion || true
fi

# shellcheck disable=SC2046
rusty-hook run --hook "${hookName}" $([ -z "$gitParams" ] && echo "" || echo "-- $gitParams")
handleRustyHookCliResult $? "${hookName}"
