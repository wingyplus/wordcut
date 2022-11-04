defmodule Wordcut.Engine.WordcutEngine do
  @moduledoc """
  TBD.
  """

  use Rustler, otp_app: :wordcut, crate: :wordcut_engine_nif

  @doc """
  Create a wordcut engine with given a dictionary `path`.
  """
  def new(_path), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Segmentation `text` into list of string.
  """
  def segment(_engine, _text), do: :erlang.nif_error(:nif_not_loaded)
end
