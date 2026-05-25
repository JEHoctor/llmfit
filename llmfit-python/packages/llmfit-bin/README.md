# llmfit-bin

Platform-specific wheel that bundles the pre-built `llmfit` binary for
[llmfit](https://github.com/AlexsJones/llmfit) — the LLM model management CLI.

This package is a workspace member of the main llmfit repository. Installing the
[`llmfit`](https://pypi.org/project/llmfit/) package is the easiest path for most
users, but `llmfit-bin` can be installed directly when only the binary is needed
(e.g. minimal virtual environments or container images that do not require the
PyO3 bindings).

After installation the `llmfit` command is available on your PATH.

```bash
llmfit --help
```

## Supported platforms

See [Rust platform support](https://doc.rust-lang.org/nightly/rustc/platform-support.html) for more information.
Refer to the the [upstream llmfit project](https://github.com/AlexsJones/llmserve?tab=readme-ov-file#llmserve) for authoritative requirements.

| Platform | Architecture | Requirements |
|---|---|---|
| Linux (glibc) | x86_64 | kernel ≥ 3.2, glibc ≥ 2.17 |
| Linux (glibc) | aarch64 | kernel ≥ 4.1, glibc ≥ 2.17 |
| Linux (musl) | x86_64 | musl ≥ 1.2.5 |
| Linux (musl) | aarch64 | musl ≥ 1.2.5 |
| macOS | x86_64 (Intel) | macOS ≥ 10.12 |
| macOS | arm64 (Apple Silicon) | macOS ≥ 11.0 |
| Windows | x86_64 | Windows 10+ or Windows Server 2016+ |
| Windows | ARM64 | |

## Version correspondence

The version of this package always matches the upstream llmfit release tag
(with the leading `v` stripped). `llmfit-bin==0.9.15` contains `v0.9.15` of the
upstream binary.

## License

The `llmfit` binary is the work of
[Alex Jones](https://github.com/AlexsJones) and contributors, released under
the MIT License. See [LICENSE](LICENSE) for details.
