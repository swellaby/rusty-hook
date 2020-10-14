# Changelog

## v0.12.0

### Added
* Make git params available to hook scripts via inline token [#122](https://github.com/swellaby/rusty-hook/pull/122)
* Make git params available to hook scripts via environment variable [#123](https://github.com/swellaby/rusty-hook/pull/123)
* Support specifying multiple commands for hook as an array [#116](https://github.com/swellaby/rusty-hook/pull/116)

### Changed
* Switch to clap from getopts to provide a more user friendly CLI experience and to better support future features [#115](https://github.com/swellaby/rusty-hook/pull/115)
* Include a `[rusty-hook]` prefix in logging output to better distinguish between rusty-hook and script/command output
* update to latest dependency versions (minor/patch bumps) of `ci_info`, `toml`, and `nias`
