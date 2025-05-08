# MailParser

[![CI](https://github.com/migadu/ex_mail_parser/actions/workflows/ci.yml/badge.svg)](https://github.com/MigaduMail/ex_mail_parser/actions/workflows/ci.yml)
[![Build precompiled NIFs](https://github.com/MigaduMail/ex_mail_parser/actions/workflows/release.yml/badge.svg)](https://github.com/MigaduMail/ex_mail_parser/actions/workflows/release.yml)
[![Docs](https://img.shields.io/badge/hex-docs-green.svg?style=flat)](https://hexdocs.pm/ex_mail_parser)
[![Hex.pm](https://img.shields.io/hexpm/v/ex_mail_parser?color=%23714a94)](http://hex.pm/packages/ex_mail_parser)

NIF binding of [mail_parser](https://github.com/stalwartlabs/mail-parser) using [Rustler](https://github.com/rusterlium/rustler).

## Installation

The package can be installed by adding `ex_mail_parser` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:ex_mail_parser, "~> 1.0"}
  ]
end
```

## Usage

To parse an email, read the email file first, then extract parts of the email:
```elixir

raw_message = File.read!("test/fixtures/example.txt")

{:ok, attachments} = ExMailParser.extract_nested_attachments(raw_message)
# where the `attachments` is the list of attchments
{:ok,
 %ExMailParser.Header{
   subject: "Bestellung 0340/2022",
   from: "joe@example.com",
   to: "max.mustermann@example.com",
   cc: "Arno.Nuehm@example.com",
   bcc: nil,
   message_id: "f938c8c82c97302cffaf3e156e982a44.asdf@example.io",
   priority: "normal",
   x_priority: "3 (Normal)",
   references: "143217a679484627beefdc85cf812cbf@abc.ch",
   newsgroup: nil,
   reply_to: "no-reply@xxxxx.de",
   in_reply_to: "d3396300-0f92-4fa5-8f50-ef44cce1e932@migadu.com",
   content_type: "multipart/signed; protocol=\"application/x-pkcs7-signature\"; micalg=\"sha-256\"; boundary=\"----08D56E512143FF456D2DCB607E33BFA2\"",
   keywords: nil,
   comments: nil,
   mime_version: "1.0",
   list_id: nil,
   list_subscribe: nil,
   list_unsubscribe: nil,
   list_post: nil,
   list_archive: nil,
   list_help: nil,
   list_owner: nil,
   date: "2022-05-17T08:05:04Z"
 }} = ExMailParser.extract_header(raw_message)

{:ok, body_html} = ExMailParser.extract_body_html(raw_message)

{:ok, body_text} = ExMailParser.extract_body_text(raw_message)

{:ok, body_preview} = ExMailParser.extract_body_preview(raw_message)
```

## Forcing compilation

By default **you don't need Rust installed** because the library will try to download a precompiled NIF file. In case you want to force compilation set the `FORCE_BUILD` environment variable to `true`.

You also need to add Rustler to your dependencies when you want to force the compilation:

```elixir
def deps do
  [
    {:ex_mail_parser, "~> 1.0"}
    {:rustler, ">= 0.0.0", optional: true}
  ]
end
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments
This is a originally a fork of https://github.com/kloeckner-i/mail_parser. Unfortunately, the original library only parsed the attachments and my pull-request did not get merged. So I created the ex_imap_parser.
