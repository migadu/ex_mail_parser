defmodule ExMailParser do
  @moduledoc """
  NIF binding of mail_parser using Rustler.
  """

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :ex_mail_parser,
    crate: :ex_mail_parser_nif,
    mode: if(Mix.env() in [:prod, :bench], do: :release, else: :debug),
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("FORCE_BUILD") in ["1", "true"],
    version: version

  alias __MODULE__.Attachment
  alias __MODULE__.Header

  @doc """
  Parses a string containing a RFC5322 raw message and extracts all nested
  attachments.

  A best-effort is made to parse the message and if no headers are found
  `:error` is returned.

  ### Example

      iex> ExMailParser.extract_nested_attachments(raw_message)
      {:ok, [%ExMailParser.Attachment{name: "example.pdf", content_type: "application/pdf", content_bytes: "..."}]}

  """
  @spec extract_nested_attachments(String.t()) :: {:ok, [Attachment.t()]} | :error
  def extract_nested_attachments(_raw_message), do: :erlang.nif_error(:nif_not_loaded)

  @spec extract_header(String.t()) :: {:ok, [Header.t()]} | :error
  def extract_header(_raw_message), do: :erlang.nif_error(:nif_not_loaded)

  @spec extract_body_html(String.t()) :: {:ok, String.t()} | :error
  def extract_body_html(_raw_message), do: :erlang.nif_error(:nif_not_loaded)

  @spec extract_body_text(String.t()) :: {:ok, String.t()} | :error
  def extract_body_text(_raw_message), do: :erlang.nif_error(:nif_not_loaded)

  @spec extract_body_preview(String.t(), Integer.t()) :: {:ok, String.t()} | :error
  def extract_body_preview(_raw_message, _preview_len), do: :erlang.nif_error(:nif_not_loaded)
end
