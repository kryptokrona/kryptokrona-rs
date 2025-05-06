# kryptokrona-rs <!-- omit in toc -->
[![build](https://img.shields.io/github/actions/workflow/status/kryptokrona/kryptokrona-rs/ci.yml?branch=main)](https://github.com/kryptokrona/kryptokrona-rs/actions/workflows/ci.yml)
[![kryptokrona: rustc 1.78.0+](https://img.shields.io/badge/kryptokrona-rustc_1.78.0+-lightgray.svg)](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html)
[![license](https://img.shields.io/badge/license-bsd%203--clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![discord](https://img.shields.io/discord/562673808582901793?label=discord)](https://discord.gg/VTgsTGS9b7)

Kryptokrona is a decentralized blockchain from the Nordic based on CryptoNote, which forms the basis for Monero, among others. CryptoNote is a so-called “application layer” protocol further developed by TurtleCoin that enables things like: private transactions, messages and arbitrary data storage, completely decentralized.

This is a project to convert the legacy C/C++ code of Kryptokrona into Rust.

## Table of Contents <!-- omit in toc -->

- [Development Resources](#development-resources)
- [Getting Help](#getting-help)
- [Reporting Issues](#reporting-issues)
- [Versioning](#versioning)
- [Checklist before release](#checklist-before-release)
- [Help and Support](#help-and-support)
- [Copypasta for license when adding new files](#copypasta-for-license-when-adding-new-files)
- [Contributors](#contributors)
- [License](#license)

## Development Resources

- Web: kryptokrona.org
- Docs: https://docs.kryptokrona.org
- GitHub: https://github.com/kryptokrona
- Hugin: Kryptokrona group on Hugin Messenger `33909fb89783fb15b5c2df50ff7107c112c3b232681f77814c024c909c07e932`r
- It is HIGHLY recommended to join our board on Hugin Messenger if you want to contribute to stay up to date on what is happening on the project.
- 𝕏: https://x.com/kryptokrona

## Getting Help

Are you having trouble with Kryptokrona? We want to help!

- Read through all documentation on our Wiki: https://docs.kryptokrona.org

- If you are upgrading, read the release notes for upgrade instructions and "new and noteworthy" features.

- Ask a question we monitor stackoverflow.com for questions tagged with kryptokrona. You can also chat with the community on Hugin or Discord.

- Report bugs with Kryptokrona at https://github.com/kryptokrona/kryptokrona-rs/issues.

- Join the Discussion and be part of the community at Discord: https://discord.gg/VTgsTGS9b7

## Reporting Issues

Kryptokrona uses GitHub’s integrated issue tracking system to record bugs and feature requests. If you want to raise an issue, please follow the recommendations below:

- Before you log a bug, please search the issue tracker to see if someone has already reported the problem.

- If the issue doesn’t already exist, create a new issue.

- Please provide as much information as possible with the issue report. We like to know the Kryptokrona version, operating system, and JVM version you’re using.

- If you need to paste code or include a stack trace, use Markdown. ``` escapes before and after your text.

- If possible, try to create a test case or project that replicates the problem and attach it to the issue.

## Versioning

Kryptokrona uses [Semantic Versioning Standard 2.0.0](https://semver.org/).

Given a version number MAJOR.MINOR.PATCH, increment the:

- MAJOR version when you make incompatible API changes
- MINOR version when you add functionality in a backwards compatible manner
- PATCH version when you make backwards compatible bug fixes

Additional labels for pre-release and build metadata are available as extensions to the MAJOR.MINOR.PATCH format.

## Checklist before release

1. Edit versions in `Cargo.toml` files.
2. Create PR and wait for review + merge.
3. Create a tag on master `git tag v*.*.*`
4. Push the changes `git push origin <tag_name>` to trigger a new build and to publish to various package managers.

## Help and Support

For questions and support please use the channel #support in [Kryptokrona](https://discord.gg/mkRpVgDubC) Discord server. The issue tracker is for bug reports and feature discussions only.

## Copypasta for license when adding new files

Hi Kryptokrona contributor, thanks for forking and sending back Pull Requests. Extensive docs about contributing are in the works or elsewhere. For now this is the bit we need to get into all the files we touch. Please add it to the top of the files, see `src/wallet_api/main.rs` for an example.

```
// Copyright (c) 2025, The Kryptokrona Project
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification, are
// permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
//    conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list
//    of conditions and the following disclaimer in the documentation and/or other
//    materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be
//    used to endorse or promote products derived from this software without specific
//    prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
// THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
// THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## License

BSD-3 License.
