defmodule ElixirSolutions.DayFifteen do
  alias Position, as: Position

  defmodule Position do
    defstruct [:x, :y]
  end

  defmodule Combatant do
    defstruct [:side, :position, :hit_points, :attack_power, :alive]
  end

  @doc ~S"""
    iex> parse_map(Enum.join([
    ...>"#######",
    ...>"#.G.E.#",
    ...>"#E.G.E#",
    ...>"#.G.E.#",
    ...>"#######"], "\n"))
    {%{
      position(0, 0) => :wall,
      position(1, 0) => :wall,
      position(2, 0) => :wall,
      position(3, 0) => :wall,
      position(4, 0) => :wall,
      position(5, 0) => :wall,
      position(6, 0) => :wall,
      position(0, 1) => :wall,
      position(6, 1) => :wall,
      position(0, 2) => :wall,
      position(6, 2) => :wall,
      position(0, 3) => :wall,
      position(6, 3) => :wall,
      position(0, 4) => :wall,
      position(1, 4) => :wall,
      position(2, 4) => :wall,
      position(3, 4) => :wall,
      position(4, 4) => :wall,
      position(5, 4) => :wall,
      position(6, 4) => :wall
    },
    [
      combatant(:goblin, position(2, 1)),
      combatant(:elf, position(4, 1)),
      combatant(:elf, position(1, 2)),
      combatant(:goblin, position(3, 2)),
      combatant(:elf, position(5, 2)),
      combatant(:goblin, position(2, 3)),
      combatant(:elf, position(4, 3))
    ]
    }
  """

  def parse_map(input) do
    {map, combatants} =
      input
      |> String.split("\n")
      |> Enum.with_index()
      |> Enum.reduce({%{}, []}, fn {line, y}, {map, combatants} ->
        line
        |> String.codepoints()
        |> Enum.with_index()
        |> Enum.reduce({map, combatants}, fn {char, x}, {map, combatants} ->
          position = position(x, y)

          case char do
            "#" -> {Map.put(map, position, :wall), combatants}
            "E" -> {map, [combatant(:elf, position) | combatants]}
            "G" -> {map, [combatant(:goblin, position) | combatants]}
            _ -> {map, combatants}
          end
        end)
      end)

    {map, Enum.reverse(combatants)}
  end

  def position(x, y), do: %Position{x: x, y: y}

  def combatant(side, position) do
    %Combatant{
      side: side,
      position: position,
      hit_points: 200,
      attack_power: 3,
      alive: true
    }
  end
end
