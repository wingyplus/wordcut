defmodule Wordcut do
  @moduledoc """
  Thai language segmentation library for Elixir.
  """

  @default_engine Wordcut.Engine.WordcutEngine

  defstruct [:engine]

  def load(dictionary_path) when is_binary(dictionary_path) do
    %__MODULE__{engine: @default_engine.new(dictionary_path) }
  end

  def segment(%__MODULE__{engine: engine}, text) do
    @default_engine.segment(engine, text)
  end
end
