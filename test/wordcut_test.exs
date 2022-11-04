defmodule WordcutTest do
  use ExUnit.Case
  doctest Wordcut

  test "greets the world" do
    assert Wordcut.hello() == :world
  end
end
