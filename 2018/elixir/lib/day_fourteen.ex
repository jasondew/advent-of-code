defmodule ElixirSolutions.DayFourteen do
  @doc ~S"""
    Part 1

    iex> ElixirSolutions.DayFourteen.part_one("9")
    "5158916779"

    iex> ElixirSolutions.DayFourteen.part_one("5")
    "0124515891"

    iex> ElixirSolutions.DayFourteen.part_one("18")
    "9251071085"

    iex> ElixirSolutions.DayFourteen.part_one("2018")
    "5941429882"
  """
  def part_one(input, final_recipe_count \\ 10) do
    rehearsal_count = String.to_integer(input)
    initial_state = %{recipes: [3, 7], current_indexes: [0, 1]}

    Enum.reduce_while(1..(rehearsal_count + final_recipe_count), initial_state, fn _round,
                                                                                   state ->
      #    IO.inspect(state, label: to_string(_round))
      current_recipes = Enum.map(state.current_indexes, &Enum.at(state.recipes, &1))
      new_recipes = current_recipes |> Enum.sum() |> Integer.digits()

      updated_recipes = state.recipes ++ new_recipes
      recipe_count = Enum.count(updated_recipes)

      updated_indexes =
        Enum.map(Enum.zip(current_recipes, state.current_indexes), fn {recipe, index} ->
          Integer.mod(index + recipe + 1, recipe_count)
        end)

      if recipe_count >= rehearsal_count + final_recipe_count do
        {:halt,
         updated_recipes
         |> Enum.drop(rehearsal_count)
         |> Enum.take(final_recipe_count)
         |> Enum.join()}
      else
        {:cont,
         %{
           state
           | recipes: updated_recipes,
             current_indexes: updated_indexes
         }}
      end
    end)
  end

  @doc ~S"""
    Part 2

    iex> ElixirSolutions.DayFourteen.part_two("51589")
    9

    iex> ElixirSolutions.DayFourteen.part_two("01245")
    5

    iex> ElixirSolutions.DayFourteen.part_two("92510")
    18

    iex> ElixirSolutions.DayFourteen.part_two("59414")
    2018
  """

  def part_two(input) do
    target = input |> String.codepoints() |> Enum.map(&String.to_integer/1)

    digits = Enum.count(target)
    initial_state = %{recipes: [3, 7], current_indexes: [0, 1]}

    Enum.reduce_while(1..21_000_000, initial_state, fn _round, state ->
      #      IO.inspect(state, label: to_string(round))
      current_recipes = Enum.map(state.current_indexes, &Enum.at(state.recipes, &1))
      new_recipes = current_recipes |> Enum.sum() |> Integer.digits()

      updated_recipes = state.recipes ++ new_recipes
      recipe_count = Enum.count(updated_recipes)

      updated_indexes =
        Enum.map(Enum.zip(current_recipes, state.current_indexes), fn {recipe, index} ->
          Integer.mod(index + recipe + 1, recipe_count)
        end)

      last_recipes = Enum.slice(updated_recipes, -(digits + 1)..-1)

      cond do
        Enum.drop(last_recipes, -1) == target ->
          {:halt, Enum.count(updated_recipes) - digits}

        Enum.drop(last_recipes, 1) == target ->
          {:halt, Enum.count(updated_recipes) - digits}

        true ->
          {:cont,
           %{
             state
             | recipes: updated_recipes,
               current_indexes: updated_indexes
           }}
      end
    end)
  end
end
