defmodule ElixirSolutions.DayNine do
  @doc ~S"""
    Part 1

    iex> ElixirSolutions.DayNine.part_one("9 players; last marble is worth 25 points")
    32

    iex> ElixirSolutions.DayNine.part_one("10 players; last marble is worth 1618 points")
    8317

    iex> ElixirSolutions.DayNine.part_one("13 players; last marble is worth 7999 points")
    146373

    iex> ElixirSolutions.DayNine.part_one("17 players; last marble is worth 1104 points")
    2764

    iex> ElixirSolutions.DayNine.part_one("21 players; last marble is worth 6111 points")
    54718

    iex> ElixirSolutions.DayNine.part_one("30 players; last marble is worth 5807 points")
    37305
  """
  def part_one(input) do
    {player_count, final_marble_points} = parse(input)
    high_score(player_count, final_marble_points)
  end

  def part_two(input) do
    {player_count, final_marble_points} = parse(input)
    high_score(player_count, final_marble_points * 100)
  end

  defmodule Circle do
    defstruct [:left, :current, :right, :count]

    def init(marble) do
      %Circle{
        left: [],
        current: marble,
        right: [],
        count: 1
      }
    end

    def add(%__MODULE__{left: [], right: []} = circle, marble) do
      %__MODULE__{
        left: [circle.current],
        current: marble,
        right: [],
        count: circle.count + 1
      }
    end

    def add(%__MODULE__{right: []} = circle, marble) do
      [first_left | rest_left] = Enum.reverse(circle.left)

      %__MODULE__{
        left: [first_left],
        current: marble,
        right: rest_left ++ [circle.current],
        count: circle.count + 1
      }
    end

    def add(%__MODULE__{right: [right_head | right_tail]} = circle, marble) do
      %__MODULE__{
        left: [right_head | [circle.current | circle.left]],
        current: marble,
        right: right_tail,
        count: circle.count + 1
      }
    end

    def remove(circle) do
      rotated_circle = rotate(circle, 7)
      [new_current | new_right] = rotated_circle.right

      {
        rotated_circle.current,
        %__MODULE__{
          left: rotated_circle.left,
          current: new_current,
          right: new_right,
          count: circle.count - 1
        }
      }
    end

    def rotate(circle, 0), do: circle

    def rotate(%__MODULE__{left: []} = circle, times) do
      rotate(
        %__MODULE__{
          left: Enum.reverse(circle.right),
          current: circle.current,
          right: []
        },
        times
      )
    end

    def rotate(%__MODULE__{left: [first_left | rest_left]} = circle, times) do
      rotate(
        %__MODULE__{
          left: rest_left,
          current: first_left,
          right: [circle.current | circle.right],
          count: circle.count
        },
        times - 1
      )
    end
  end

  defp high_score(player_count, final_marble_points) do
    player_scores = for player_number <- 1..player_count, into: %{}, do: {player_number, 0}

    1..final_marble_points
    |> Enum.reduce({Circle.init(0), 1, player_scores}, fn marble,
                                                          {circle, current_player, player_scores} ->
      next_player = Integer.mod(current_player, player_count) + 1

      if Integer.mod(marble, 23) == 0 do
        {removed_marble, updated_circle} = Circle.remove(circle)

        {
          updated_circle,
          next_player,
          Map.update!(player_scores, current_player, &(&1 + marble + removed_marble))
        }
      else
        updated_circle = Circle.add(circle, marble)

        {
          updated_circle,
          next_player,
          player_scores
        }
      end
    end)
    |> elem(2)
    |> Map.values()
    |> Enum.max()
  end

  defp parse(input) do
    [players, "players;", "last", "marble", "is", "worth", points, "points"] =
      String.split(input, " ")

    {String.to_integer(players), String.to_integer(points)}
  end
end
