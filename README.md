This repository houses functional tests for e.g. `gitoxide` that will verify correct behavior with specific repository states/configurations. These test cases were created with git 2.35.2, xz 5.2.5, and GNU tar 1.34.

The archived git repositories live in `archives` and the scripts to create the repositories live in `scenarios`.  With a populated `archives` directory the tests can be run via `cargo test`
