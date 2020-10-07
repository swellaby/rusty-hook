#!/bin/sh

stdoutF="${TMPDIR:-/tmp}/STDOUT"
expectedStdoutF="${TMPDIR:-/tmp}/expStdout"
stderrF="${TMPDIR:-/tmp}/STDERR"

# handleRustyHookCliResult Tests

tearDown() {
  rm -f "${stdoutF}" || true
  rm -f "${expectedStdoutF}" || true
  rm -f "${stderrF}" || true
}

testReturnsZeroWhenRustyHookExitCodeIsZero() {
  (
    handleRustyHookCliResult 0 "" >"${stdoutF}" 2>"${stderrF}"
  )
  act=$?
  act=$?
  assertEquals 0 ${act}
}

testHandlesNonPreCommitHookCorrectlyWhenRustyHookExitCodeIsNoConfigFile() {
  hook="pre-push"
  noConfigFileExitCode=3
  (
    handleRustyHookCliResult ${noConfigFileExitCode} "${hook}" >"${stdoutF}" 2>"${stderrF}"
  )
  act=$?
  output=$(cat "${stdoutF}")
  assertEquals 0 ${act}
  assertEquals "" "${output}"
}

testHandlesPreCommitHookCorrectlyWhenRustyHookExitCodeIsNoConfigFile() {
  hook="pre-commit"
  {
    echo "[rusty-hook] rusty-hook git hooks are configured, but no config file was found";
    echo "[rusty-hook] In order to use rusty-hook, your project must have a config file";
    echo "[rusty-hook] See https://github.com/swellaby/rusty-hook#configure for more information about configuring rusty-hook"
    echo;
    echo "[rusty-hook] If you were trying to remove rusty-hook, then you should also delete the git hook files to remove this warning";
    echo "[rusty-hook] See https://github.com/swellaby/rusty-hook#removing-rusty-hook for more information about removing rusty-hook from your project";
    echo;

  } >> "${expectedStdoutF}"
  expOutput=$(cat "${expectedStdoutF}")
  noConfigFileExitCode=3
  (
    handleRustyHookCliResult 3 "${hook}" >"${stdoutF}" 2>"${stderrF}"
  )
  act=$?
  output=$(cat "${stdoutF}")
  assertEquals 0 ${act}
  assertEquals "${expOutput}" "${output}"
}

testHandlesCorrectlyWhenRustyHookExitCodeIsOne() {
  hook="pre-push"
  echo "[rusty-hook] Configured hook command failed" >> "${expectedStdoutF}"
  echo "[rusty-hook] ${hook} hook rejected" >> "${expectedStdoutF}"
  expOutput=$(cat "${expectedStdoutF}")
  expExitCode=1
  (
    handleRustyHookCliResult ${expExitCode} "${hook}" >"${stdoutF}" 2>"${stderrF}"
  )
  act=$?
  output=$(cat "${stdoutF}")
  assertEquals ${expExitCode} ${act}
  assertEquals "${expOutput}" "${output}"
}

# shellcheck source=src/hook_files/cli.sh
. "$(dirname "$0")"/../../src/hook_files/cli.sh
# shellcheck source=tests/hook_files/shunit2.sh
. "$(dirname "$0")"/shunit2.sh
