defmodule ExMailParserTest do
  use ExUnit.Case

  doctest ExMailParser, except: [extract_nested_attachments: 1]

  test "extracts attachments from raw message" do
    raw_message = File.read!("test/fixtures/example.txt")

    assert {:ok,
            [
              %ExMailParser.Attachment{
                name: "Best 340 Klöckner FL-Stahl.pdf",
                content_type: "application/pdf",
                content_bytes: pdf_content_bytes
              },
              %ExMailParser.Attachment{
                name: "smime.p7s",
                content_type: "application/x-pkcs7-signature",
                content_bytes: "redacted"
              }
            ]} = ExMailParser.extract_nested_attachments(raw_message)

    assert pdf_content_bytes == File.read!("test/fixtures/sample.pdf")
  end

  test "extracts headers from raw message" do
    raw_message = File.read!("test/fixtures/example.txt")

    assert {:ok,
            %ExMailParser.Header{
              bcc: nil,
              cc: "Arno.Nuehm@example.com",
              date: "2022-05-17T08:05:04Z",
              from: "joe@example.com",
              subject: "Bestellung 0340/2022",
              to: "max.mustermann@example.com"
            }} = ExMailParser.extract_header(raw_message)
  end

  test "returns error if parsing fails" do
    assert :error = ExMailParser.extract_nested_attachments("")
  end

  test "extracts html body from raw message" do
    raw_message = File.read!("test/fixtures/example2.txt")

    assert {:ok,
            "<html><p>I was thinking about quitting the &ldquo;exporting&rdquo; to focus just on the &ldquo;importing&rdquo;,</p><p>but then I thought, why not do both? &#x263A;</p></html>"} =
             ExMailParser.extract_body_html(raw_message)
  end

  test "extracts text body from raw message" do
    raw_message = File.read!("test/fixtures/example2.txt")

    assert {:ok,
            "I was thinking about quitting the “exporting” to focus just on the “importing”,\nbut then I thought, why not do both? ☺\n"} =
             ExMailParser.extract_body_text(raw_message)
  end

  test "extracts text preview from raw message" do
    raw_message = File.read!("test/fixtures/example2.txt")

    assert {:ok, "I was thinking about quitting the “exporting..."} =
             ExMailParser.extract_body_preview(raw_message, 50)
  end
end
