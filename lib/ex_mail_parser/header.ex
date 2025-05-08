defmodule ExMailParser.Header do
  @moduledoc """
  A message header.
  """

  @type t :: %__MODULE__{
          subject: String.t() | nil,
          from: String.t() | nil,
          to: String.t(),
          cc: String.t() | nil,
          bcc: String.t() | nil,
          message_id: String.t() | nil,
          priority: String.t() | nil,
          x_priority: String.t() | nil,
          references: String.t() | nil,
          newsgroup: String.t() | nil,
          reply_to: String.t() | nil,
          in_reply_to: String.t() | nil,
          content_type: String.t() | nil,
          keywords: String.t() | nil,
          comments: String.t() | nil,
          mime_version: String.t() | nil,
          list_id: String.t() | nil,
          list_subscribe: String.t() | nil,
          list_unsubscribe: String.t() | nil,
          list_post: String.t() | nil,
          list_archive: String.t() | nil,
          list_help: String.t() | nil,
          list_owner: String.t() | nil,
          date: String.t() | nil
        }

  defstruct [
    :subject,
    :from,
    :to,
    :cc,
    :bcc,
    :message_id,
    :priority,
    :x_priority,
    :references,
    :newsgroup,
    :reply_to,
    :in_reply_to,
    :content_type,
    :keywords,
    :comments,
    :mime_version,
    :list_id,
    :list_subscribe,
    :list_unsubscribe,
    :list_post,
    :list_archive,
    :list_help,
    :list_owner,
    :date
  ]
end
