# rusty-hook
Git hook utility for Rust codebases that lets you run any script for any git hook. 

Functional, but still in Beta! 

[![Version Badge][version-badge]][crate url]
[![Downloads Badge][downloads-badge]][crate url]
[![License Badge][license-badge]][crate url]  

[![Linux CI Badge][linux-ci-badge]][linux-ci-url]
[![Mac CI Badge][mac-ci-badge]][mac-ci-url]
[![Windows CI Badge][windows-ci-badge]][windows-ci-url]  

[![Test Results Badge][tests-badge]][tests-url]
[![Coverage Badge][coverage-badge]][coverage-url]

## Installing
We'll add binary releases soon, but for now the best way to install `rusty-hook` is to use `cargo`:

```sh
cargo install rusty-hook
```

## Initialize
Run the `init` command in any git directory to set it up:

```sh
rusty-hook init
```

This will ensure that all of the client side git hooks are available, and it will create a `rusty-hook` configuration file if one does not already exist.

## Configure
You define your desired [git hook][git hooks] configuration in the `rusty-hook` configuration file (a TOML file named `.rusty-hook.toml` or `rusty-hook.toml`).

Here's an example `rusty-hook` configuration that leverages multiple [git hooks][git hooks], including the [pre-commit][pre-commit hook] and the [pre-push][pre-push hook] hooks:

```toml
[hooks]
pre-commit = "cargo test"
pre-push = "cargo fmt -- --check"
post-commit = "echo yay"

[logging]
verbose = true
```
### Hooks
Under the `[hooks]` table, you can add an entry for any and every git hook you want to run by adding a key using the name of the [git hook][git hooks], and then specify the command/script you want to run for that hook. Whenever that git hook is triggered, `rusty-hook` will run your specified command!

### Logging
Under the `[logging]` table, you can control whether to log the output of running your specified hook commands. By default `rusty-hook` will log the results of your hook script, but you can disable this behavior by setting the `verbose` key to `false`:

```toml
[logging]
verbose = false
```

## Alternatives
There's a few other git hook utilities available on [crates.io][cratesio], but none of them quite suited our needs so we made rusty-hook!

* [cargo-husky][cargo-husky crate]
* [shiba][shiba crate]
* [git_hooks][git_hooks crate]

## Contributions
All contributions are welcome and appreciated! Check out our [Contributing Guidelines][contributing] for more information about opening issues, developing, and more.

[version-badge]: https://img.shields.io/crates/v/rusty-hook.svg?style=flat-square
[license-badge]: https://img.shields.io/crates/l/rusty-hook.svg?style=flat-square
[downloads-badge]: https://img.shields.io/crates/d/rusty-hook.svg?style=flat-square
[crate url]: https://crates.io/crates/rusty-hook
[linux-ci-badge]: https://img.shields.io/azure-devops/build/swellaby/opensource/49.svg?label=linux%20build&style=flat-square
[linux-ci-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=49
[mac-ci-badge]: https://img.shields.io/azure-devops/build/swellaby/opensource/54.svg?label=mac%20build&style=flat-square
[mac-ci-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=54
[windows-ci-badge]: https://img.shields.io/azure-devops/build/swellaby/opensource/56.svg?label=windows%20build&style=flat-square
[windows-ci-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=56
[coverage-badge]: https://img.shields.io/azure-devops/coverage/swellaby/opensource/49.svg?style=flat-square
[coverage-url]: https://codecov.io/gh/swellaby/rusty-hook
[tests-badge]: https://img.shields.io/azure-devops/tests/swellaby/opensource/49.svg?label=unit%20tests&style=flat-square
[tests-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=49&view=ms.vss-test-web.build-test-results-tab
[git hooks]: https://git-scm.com/docs/githooks#_hooks
[pre-commit hook]: https://git-scm.com/docs/githooks#_pre_commit
[pre-push hook]: https://git-scm.com/docs/githooks#_pre_push
[cargo-husky crate]: https://crates.io/crates/cargo-husky
[shiba crate]: https://crates.io/crates/shiba
[git_hooks crate]: https://crates.io/crates/git_hooks
[cratesio]: https://crates.io
[contributing]: .github/CONTRIBUTING.md
