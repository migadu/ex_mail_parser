# Changelog
## [1.1.0] - 2025-05-08

- Added more fields to be extracted from the header

## [1.0.0] - 2025-04-23

- Add basic header parsing
- Add extract_body_html/text/preview
- Forked from https://github.com/kloeckner-i/mail_parser/ and renamed to ex_mail_parser

## [0.7.0] - 2023-05-22

- Add support for OTP 26

## [0.6.1] - 2023-03-30

- Update dependencies

## [0.6.0] - 2023-02-03

- Update rustler_precompiled to 0.6.0
- Add precompiled binaries for the `riscv64gc-unknown-linux-gnu` target
- Drop version 2.14 (targeting OTP 21) from the supported NIF versions
- Update GitHub action workflows

## [0.5.1] - 2023-01-25

- Revert "Drop NIF version 2.14"

## [0.5.0] - 2023-01-25

- Drop NIF version 2.14
- Simplify release workflow
- Bump rustler to 0.27.0
- Bump mail-parser to 0.8.0

## [0.4.1] - 2022-11-03

- Bump `mail-parser` to 0.7.0
- Bump `rustler` to 0.26.0

## [0.4.0] - 2022-08-01

- Upgrade `mail-parser` to 0.5.0
- Bump `cross` to v0.2.4

## [0.3.4] - 2022-07-04

- Fix the precompilation build for targets using `cross` by adding a `Cross.toml` file with a setting telling to read the `RUSTLER_NIF_VERSION` env var from the host machine
- Bump `cross` to v0.2.2

## [0.3.3] - 2022-05-24

- Bump `:rustler_precompiled` to v0.5.1
- Fix typo in project description

## [0.3.2] - 2022-05-23

- Use fork of rustler_precompiled which adds `aarch64-unknown-linux-musl` to the available targets

## [0.3.1] - 2022-05-23

- Precompile NIF for `aarch64-unknown-linux-musl`

## [0.3.0] - 2022-05-18

- Return `:content_bytes` as Erlang binary instead of base64 encoded string
- Eliminate call to `unwrap()`
- Publish package to hex

## [0.2.0] - 2022-05-17

- Disable `mail_parser`s default features
- Rename `MailParser.parse` to `MailParser.extract_nested_attachments`
- Rename NIF package
- Add checksum file

## [0.1.0] - 2022-05-17

[unreleased]: https://github.com/kloeckner-i/mail_parser/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.7.0
[0.6.1]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.6.1
[0.6.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.6.0
[0.5.1]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.5.1
[0.5.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.5.0
[0.4.1]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.4.1
[0.4.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.4.0
[0.3.4]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.3.4
[0.3.3]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.3.3
[0.3.2]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.3.2
[0.3.1]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.3.1
[0.3.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.3.0
[0.2.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.2.0
[0.1.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.1.0
