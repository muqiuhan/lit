<div align="center">

<img src="./.github/logo.png" height="150px">

<h1> Lit </h1>

<p> A minimalism cross-platform git-like version control system written in Rust </p>

<img src="https://github.com/muqiuhan/lit/actions/workflows/ci.yaml/badge.svg">

<p> <mark> 🚧 Working In Progress. </mark> </p>

</div>

## Installation
- `make install`
- `make uninstall`

[Makefile](./Makefile):
```makefile
.PHONY: build install uninstall build.release fmt check fix test

build :
	@cargo build

install : build.release
	@cargo install --path .

uninstall :
	@cargo uninstall

build.release :
	@cargo build --release

fmt:
	@cargo fmt

check:
	@cargo clippy

fix:
	@cargo clippy --fix --allow-staged

test:
	@cargo test -- --test-threads=1
```

## Usage
```
lit 0.1.0
A minimalism cross-platform git-like version control system written in Rust

USAGE:
    lit <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cat-file    Provide contents or details of repository objects
    help        Prints this message or the help of the given subcommand(s)
    init        Create an empty lit repository or reinitialize an existing one
```


## FAQ

### Packfiles?

Lit just implemented __loose objects__. Git has a second object storage mechanism called __packfile__. The Packfiles are much more efficient, but more complex tan loose objects.

A packfile is a compilation of loose objects (like a tar) but some are stored as deltas (as transformation of another object).

In Git, the packfile is stored in `.git/objects/pack`, it has a `.pack` extension, and is accompanied by an index file of the same name with the `idx` extension.

If you want to convert a packfile to loose objects format (to play with lit on an existing repo), here is the solution:

1. Move the packfile outside the gitdir:
	> `mv .git/objects/pack/pack-xxxxxx.pack .`
2. Cat it and pipe the result to `git unpack-objects`
	> `cat pack-xxxxx.pack | git unpack-objects`

## Progress

- [ ] add                   Add file contents to the index
- [x] init                  Create an empty lit repository or reinitialize an existing one
- [ ] log                   Show commit logs
- [ ] rm                    Remove files from the working tree and from the index
- [ ] tagging               Create, list, delete or verify a tag object signed with GPG
- [ ] status                Show the working tree status
- [x] cat-file              Provide content or type and size information for repository objects
- [ ] check-ignore          Debug gitignore / exclude files
- [ ] checkout              Switch branches or restore working tree files
- [ ] commit                Record changes to the repository
- [x] hash-object           Compute object ID and optionally create an object from a file
- [ ] ls-files              Show information about files in the index and the working tree
- [ ] ls-tree               List the contents of a tree object
- [ ] rev-parse             Pick out and massage parameters
- [ ] show-ref              List references in a local repository

## Dependencies

| Name                                                          | License            | Description                                                                      |
| ------------------------------------------------------------- | ------------------ | -------------------------------------------------------------------------------- |
| [structopt](https://github.com/TeXitoi/structopt)             | Apache 2.0 and MIT | Parse command line arguments by defining a struct.                               |
| [rust-ini](https://github.com/zonyitoo/rust-ini)              | MIT                | INI file parser in Rust                                                          |
| [colog](https://github.com/muqiuhan/rust-colog)               | LGPL 3.0           | A simple color-coded logging implementation for the standard rust logging system |
| [log](https://github.com/rust-lang/log)                       | Apache 2.0 and MIT | Logging implementation for Rust                                                  |
| [flate2-rs](https://github.com/rust-lang/flate2-rs)           | Apache 2.0 and MIT | DEFLATE, gzip, and zlib bindings for Rust                                        |
| [sha1](https://github.com/RustCrypto/hashes/tree/master/sha1) | Apache 2.0 and MIT | Pure Rust implementation of the SHA-1 hash function.                             |


## [LICENSE](./LICENSE)

Copyright (C) 2023 Muqiu Han

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU Library General Public
License as published by the Free Software Foundation; either
version 2 of the License, or (at your option) any later version.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Library General Public License for more details.

You should have received a copy of the GNU Library General Public
License along with this library; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
