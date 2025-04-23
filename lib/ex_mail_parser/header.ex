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
          date: String.t() | nil
        }

  defstruct [:subject, :from, :to, :cc, :bcc, :date]
end
