defmodule Conversions do
  def to_integers(input, delimiter \\ "\n") do
    input
    |> String.split(delimiter)
    |> Enum.map(&String.to_integer/1)
  end
end
