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

## Quick Start
1. Add `rusty-hook` as a dev dependency in your Cargo.toml file
2. Run `cargo test` (to build your dev dependencies, including `rusty-hook`)
3. Update the generated `.rusty-hook.toml` file with the commands you want to run
4. Run `git commit` (or equivalent to trigger your git hook)!
    - You may also want to have your hook script fail (for example add a failing test if your commit hook is `cargo test`) to see the hooks be enforced.
    - **note the very first (and only) time you do this will take an extra ~30 seconds or so to finalize the setup**

## Setup
Just add `rusty-hook` as a dev dependency in your Cargo.toml file:

```toml
[dev-dependencies]
rusty-hook = "^0.10.1"
```

## Initialize
When you add `rusty-hook` as a dev-dependency in your project, it will automatically configure the git hooks once it is built (for example the first time you run `cargo test`).

This will ensure that all of the client side git hooks are setup and available, and it will create a `rusty-hook` configuration file if one does not already exist.

The git hook script will ensure that the `rusty-hook` cli is available, so the very first time a git hook is triggered on your machine you will see a message indicating that the `rusty-hook` setup is being finalized which may take ~30 seconds or so:
```sh
Finalizing rusty-hook configuration...
This may take a few seconds...
```

### (Optional) Install
You can also install the `rusty-hook` cli with cargo:
```sh
cargo install rusty-hook
```

You can optionally manually initialize any git directory by running the `init` command in any git directory to set it up:

```sh
rusty-hook init
```

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

## Removing rusty-hook
We'll be sad to see you go, but here's what to do if you'd like to remove `rusty-hook` from your project. 

1. Remove the `rusty-hook` dev dependency from the `Cargo.toml` file in your project.
2. Remove the `.rusty-hook.toml` configuration file from your project.
3. Remove the git hook scripts that were placed in the git hooks directory in your local project workspace (this is typically in the `.git/hooks/` directory). Note that if you were using `rusty-hook` version `0.9.1` or newer and you skip this step, then the git hooks will still be invoked as part of your git workflow and you will see the following warning message on git commit:
```
rusty-hook git hooks are configured, but no config file was found
In order to use rusty-hook, your project must have a config file
See https://github.com/swellaby/rusty-hook#configure for more information about configuring rusty-hook

If you were trying to remove rusty-hook, then you should also delete the git hook files to remove this warning
See https://github.com/swellaby/rusty-hook#removing-rusty-hook for more information about removing rusty-hook from your project
```

Please also consider [opening an issue][create-issue] to report any bugs/problems you experienced, missing features, etc. so that we can work on improving `rusty-hook`!

[version-badge]: https://img.shields.io/crates/v/rusty-hook.svg?style=flat-square
[license-badge]: https://img.shields.io/crates/l/rusty-hook.svg?style=flat-square
[downloads-badge]: https://img.shields.io/crates/d/rusty-hook.svg?style=flat-square
[crate url]: https://crates.io/crates/rusty-hook
[linux-ci-badge]: https://img.shields.io/azure-devops/build/swellaby/opensource/49/master.svg?label=linux%20build&style=flat-square
[linux-ci-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=49
[mac-ci-badge]: https://img.shields.io/azure-devops/build/swellaby/opensource/54/master.svg?label=mac%20build&style=flat-square
[mac-ci-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=54
[windows-ci-badge]: https://img.shields.io/azure-devops/build/swellaby/opensource/56/master.svg?label=windows%20build&style=flat-square
[windows-ci-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=56
[coverage-badge]: https://img.shields.io/azure-devops/coverage/swellaby/opensource/49/master.svg?style=flat-square
[coverage-url]: https://codecov.io/gh/swellaby/rusty-hook
[tests-badge]: https://img.shields.io/azure-devops/tests/swellaby/opensource/49/master.svg?label=unit%20tests&style=flat-square
[tests-url]: https://dev.azure.com/swellaby/OpenSource/_build/latest?definitionId=49&view=ms.vss-test-web.build-test-results-tab
[git hooks]: https://git-scm.com/docs/githooks#_hooks
[pre-commit hook]: https://git-scm.com/docs/githooks#_pre_commit
[pre-push hook]: https://git-scm.com/docs/githooks#_pre_push
[cargo-husky crate]: https://crates.io/crates/cargo-husky
[shiba crate]: https://crates.io/crates/shiba
[git_hooks crate]: https://crates.io/crates/git_hooks
[cratesio]: https://crates.io
[contributing]: .github/CONTRIBUTING.md
[create-issue]: https://github.com/swellaby/rusty-hook/issues/new/choose
