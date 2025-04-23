defmodule ExMailParser.MessagePart do
  @moduledoc """
  A message part.
  """

  @type t :: %__MODULE__{
          headers: String.t() | nil,
          is_encoding_problem: Boolean.t() | nil,
          body: String.t(),
          encoding: String.t() | nil,
          offset_header: Integer.t() | nil,
          offset_body: Integer.t() | nil,
          offset_end: Integer.t() | nil
        }

  defstruct [
    :headers,
    :is_encoding_problem,
    :body,
    :encoding,
    :offset_header,
    :offset_body,
    :offset_end
  ]
end
