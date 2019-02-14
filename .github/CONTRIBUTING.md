# Contributing
All contributions are welcome and appreciated! 

## Opening Issues
Need to open an issue? Click [here][create-issue] or use one of the below links:

- [Report a bug][create-bug]
- [Request an enhancement or new feature][create-feature-request]
- [Ask a question][create-question]

## Developing
All that is needed to work with this repo is [Rust][rust] and your favorite editor or IDE, although we recommend [VS Code][vscode]. You can run the `dev_setup` script described below to set up Rust in your environment if you do not already have Rust.

### Building
To build and/or work on this project:

Clone the repo, change into the directory where you cloned the directory and then run the dev setup script for your platform

```sh     
git clone https://github.com/swellaby/rusty-hook.git
cd rusty-hook
```  

For Linux, Mac, and Windows WSL:
```
./scripts/dev_setup.sh
```

For Windows:
```
.\scripts\dev_setup.ps1
```

### Submitting Changes
Swellaby members should create a branch within the repository, make changes there, and then submit a Pull Request. 

Outside contributors should fork the repository, make changes in the fork, and then submit a Pull Request. Check out the [GitHub Forking Projects Guide][fork-guide-url] for more info.

#### PR Validation
When you create a Pull Request to merge changes, the [PR Validation Build Jobs][ci-pipeline-url] in Azure Pipelines will be triggered automatically to run test coverage, format, and lint checks. 

### Tests
There are suites of [unit tests][unit-test] that validate individual functions. The unit tests are defined in separate files that reside within the `src` directory alongside the application code. The tests are placed in a separate file by leveraging the Rust module [path attribute][path attribute] on the test module declaration in each source file. We know the Rust convention is to place unit tests in the same file, but we like this way better!

The tests can be executed via `cargo`

Run the unit tests:
```sh
cargo test
```  

You must write corresponding unit tests for any code you add or modify, and all tests must pass before those changes can be merged back to the master branch.

### Code Coverage
We're currently using `kcov` for code coverage, more detailed documentation coming soon.

We've got 100% [code coverage][codecov project] (which intentionally excludes `main.rs`) that must be maintained for all new and modified code.

### Linting
Naturally, we use [clippy][clippy] for linting.

To run the linter, use cargo:
```sh
cargo clippy
```  

### Code Formatting
We use [rustfmt][rustfmt] to ensure consistent formatting of the Rust code. 

To run rustfmt, use cargo:
```sh
cargo fmt
```

### git hooks
This repo utilizes itself for client side [git hooks][git hooks]. 

The [git pre-commit hook][pre-commit hook] is configured to run the unit tests so when you run a `git commit`, the pre-commit hook will trigger those tests. If any of the tests fail then your commit will be rejected, and you will need to fix the issue(s) before attempting to commit again.  

You can optionally skip this hook by including the `-n` switch (i.e. `git commit -n -m "..."`) if you are only trying to commit non-code content, like a markdown or TOML file.

The [git pre-push hook][pre-push hook] is configured to run a `rustfmt` check to ensure that all the Rust files are formatted correctly. If any of the files are *not* formatted correctly, then the push will fail, and you will need to fix the issue(s) before attempting to push again.

You can optionally skip this hook by including the `--no-verify` switch (i.e. `git push --no-verify`)  

<br /> 

[Back to Top][top]


[top]: CONTRIBUTING.md#contributing
[create-issue]: https://github.com/swellaby/rusty-hook/issues/new/choose
[create-bug]: https://github.com/swellaby/rusty-hook/issues/new?template=01_BUG.md
[create-feature-request]: https://github.com/swellaby/rusty-hook/issues/new?template=02_FEATURE_REQUEST.md
[create-question]: https://github.com/swellaby/rusty-hook/issues/new?template=03_QUESTION.md
[vscode]: https://code.visualstudio.com/
[rust]: https://www.rust-lang.org/
[fork-guide-url]: https://guides.github.com/activities/forking/
[ci-pipeline-url]: https://dev.azure.com/swellaby/OpenSource/_build?definitionId=49
[unit-test]: ../src
[path attribute]: https://doc.rust-lang.org/reference/items/modules.html#path-attribute
[codecov project]: https://codecov.io/gh/swellaby/rusty-hook
[clippy]: https://github.com/rust-lang/rust-clippy
[rustfmt]: https://github.com/rust-lang/rustfmt
[git hooks]: https://git-scm.com/docs/githooks#_hooks
[pre-commit hook]: https://git-scm.com/docs/githooks#_pre_commit
[pre-push hook]: https://git-scm.com/docs/githooks#_pre_push
