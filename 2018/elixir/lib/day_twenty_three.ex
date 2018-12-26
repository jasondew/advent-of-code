defmodule ElixirSolutions.DayTwentyThree do
  defmodule Position do
    defstruct [:x, :y, :z]

    def distance(a, b) do
      abs(a.x - b.x) + abs(a.y - b.y) + abs(a.z - b.z)
    end
  end

  defmodule Bot do
    defstruct [:position, :strength]

    def distance_between(bot, other_bot) do
      Position.distance(bot.position, other_bot.position)
    end
  end

  defimpl Inspect, for: Bot do
    def inspect(bot, _opts) do
      "pos=<#{to_string(bot.position.x)},#{to_string(bot.position.y)},#{to_string(bot.position.z)}>, r=#{
        bot.strength
      }}"
    end
  end

  @doc ~S"""
    iex> part_one("pos=<0,0,0>, r=4
    ...>pos=<1,0,0>, r=1
    ...>pos=<4,0,0>, r=3
    ...>pos=<0,2,0>, r=1
    ...>pos=<0,5,0>, r=3
    ...>pos=<0,0,3>, r=1
    ...>pos=<1,1,1>, r=1
    ...>pos=<1,1,2>, r=1
    ...>pos=<1,3,1>, r=1")
    7
  """

  def part_one(input) do
    bots = parse(input)

    head_honcho =
      bots
      |> Enum.sort_by(&(-&1.strength))
      |> List.first()

    Enum.count(bots, fn bot ->
      Bot.distance_between(bot, head_honcho) <= head_honcho.strength
    end)
  end

  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse_bot/1)
  end

  defp parse_bot(line) do
    [position_part, strength_part] = String.split(line, ", ", parts: 2)
    %Bot{position: parse_position(position_part), strength: parse_strength(strength_part)}
  end

  defp parse_position("pos=" <> triplet) do
    [x, y, z] = triplet |> String.split(~w[< , >], trim: true) |> Enum.map(&String.to_integer/1)
    %Position{x: x, y: y, z: z}
  end

  def parse_strength("r=" <> strength_string), do: String.to_integer(strength_string)
end
