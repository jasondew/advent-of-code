defmodule DayFourteen do
  @doc """
    Part 1

      iex> DayFourteen.part_one("10 ORE => 10 A\\n1 ORE => 1 B\\n7 A, 1 B => 1 C\\n7 A, 1 C => 1 D\\n7 A, 1 D => 1 E\\n7 A, 1 E => 1 FUEL")
      31

      iex> DayFourteen.part_one("9 ORE => 2 A\\n8 ORE => 3 B\\n7 ORE => 5 C\\n3 A, 4 B => 1 AB\\n5 B, 7 C => 1 BC\\n4 C, 1 A => 1 CA\\n2 AB, 3 BC, 4 CA => 1 FUEL")
      165

      iex> DayFourteen.part_one("157 ORE => 5 NZVS\\n165 ORE => 6 DCFZ\\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\\n179 ORE => 7 PSHF\\n177 ORE => 5 HKGWZ\\n7 DCFZ, 7 PSHF => 2 XJWVT\\n165 ORE => 2 GPVTF\\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")
      13312

      iex> DayFourteen.part_one("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\\n17 NVRVD, 3 JNWZP => 8 VPVL\\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\\n22 VJHF, 37 MNCFX => 5 FWMGM\\n139 ORE => 4 NVRVD\\n144 ORE => 7 JNWZP\\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\\n145 ORE => 6 MNCFX\\n1 NVRVD => 8 CXFTF\\n1 VJHF, 6 MNCFX => 4 RFSQX\\n176 ORE => 6 VJHF")
      180697

      iex> DayFourteen.part_one("171 ORE => 8 CNZTR\\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\\n114 ORE => 4 BHXH\\n14 VRPVC => 6 BMBT\\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\\n5 BMBT => 4 WPTQ\\n189 ORE => 9 KTJDG\\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\\n12 VRPVC, 27 CNZTR => 2 XDBXC\\n15 KTJDG, 12 BHXH => 5 XCVML\\n3 BHXH, 2 VRPVC => 7 MZWV\\n121 ORE => 7 VRPVC\\n7 XCVML => 6 RJRHP\\n5 BHXH, 4 VRPVC => 5 LTCX")
      2210736
  """
  def part_one(input) do
    input
    |> parse()
    |> ore_for_fuel()
  end

  @doc """
    Part 2

      iex> DayFourteen.part_two("157 ORE => 5 NZVS\\n165 ORE => 6 DCFZ\\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\\n179 ORE => 7 PSHF\\n177 ORE => 5 HKGWZ\\n7 DCFZ, 7 PSHF => 2 XJWVT\\n165 ORE => 2 GPVTF\\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")
      82892753

      iex> DayFourteen.part_two("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\\n17 NVRVD, 3 JNWZP => 8 VPVL\\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\\n22 VJHF, 37 MNCFX => 5 FWMGM\\n139 ORE => 4 NVRVD\\n144 ORE => 7 JNWZP\\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\\n145 ORE => 6 MNCFX\\n1 NVRVD => 8 CXFTF\\n1 VJHF, 6 MNCFX => 4 RFSQX\\n176 ORE => 6 VJHF")
      5586022

      iex> DayFourteen.part_two("171 ORE => 8 CNZTR\\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\\n114 ORE => 4 BHXH\\n14 VRPVC => 6 BMBT\\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\\n5 BMBT => 4 WPTQ\\n189 ORE => 9 KTJDG\\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\\n12 VRPVC, 27 CNZTR => 2 XDBXC\\n15 KTJDG, 12 BHXH => 5 XCVML\\n3 BHXH, 2 VRPVC => 7 MZWV\\n121 ORE => 7 VRPVC\\n7 XCVML => 6 RJRHP\\n5 BHXH, 4 VRPVC => 5 LTCX")
      460664
  """

  def part_two(input) do
    input
    |> parse()
    |> maximum_fuel(1_000_000_000_000)
  end

  ## PRIVATE FUNCTIONS

  defp maximum_fuel(reactions, ore) do
    guess = div(ore, ore_for_fuel(reactions, 1))
    f = fn x -> ore - ore_for_fuel(reactions, x) end

    binary_search({guess, guess * 2}, f)
  end

  defp binary_search({x, x}, _f), do: x

  defp binary_search({low, high}, f) do
    midpoint = div(low + high, 2)

    midpoint =
      if midpoint == low do
        high
      else
        midpoint
      end

    if f.(midpoint) > 0 do
      binary_search({midpoint, high}, f)
    else
      binary_search({low, midpoint - 1}, f)
    end
  end

  defp ore_for_fuel(reactions, amount \\ 1) do
    reactions
    |> ore_required({amount, "FUEL"})
    |> elem(0)
  end

  defp ore_required(_reactions, _what, leftovers \\ %{}, indent \\ "")

  defp ore_required(_reactions, {amount_required, "ORE"}, leftovers, _indent) do
    {amount_required, leftovers}
  end

  defp ore_required(reactions, {amount_required, chemical}, leftovers, indent) do
    amount_available = Map.get(leftovers, chemical, 0)

    {amount_required, leftovers} =
      if amount_available >= amount_required do
        {0, Map.put(leftovers, chemical, amount_available - amount_required)}
      else
        {amount_required - amount_available, Map.delete(leftovers, chemical)}
      end

    if amount_required > 0 do
      %{amount: amount_produced, ingredients: ingredients} =
        Map.get(reactions, chemical)

      batches_needed = ceil(amount_required / amount_produced)
      leftover = batches_needed * amount_produced - amount_required

      leftovers =
        if leftover > 0 do
          Map.update(leftovers, chemical, leftover, &(&1 + leftover))
        else
          leftovers
        end

      {total, leftovers} =
        Enum.reduce(ingredients, {0, leftovers}, fn {ingredient,
                                                     ingredient_amount},
                                                    {total_ore, leftovers} ->
          {ore, leftovers} =
            ore_required(
              reactions,
              {ingredient_amount * batches_needed, ingredient},
              leftovers,
              "#{indent}  "
            )

          {ore + total_ore, leftovers}
        end)

      {total, leftovers}
    else
      {0, leftovers}
    end
  end

  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse_reaction/1)
    |> Enum.reduce(%{}, fn {{chemical, amount}, ingredients}, map ->
      Map.put(map, chemical, %{amount: amount, ingredients: ingredients})
    end)
  end

  defp parse_reaction(string) do
    [ingredients, output] = String.split(string, " => ")

    {
      parse_quantity(output),
      ingredients
      |> String.split(", ")
      |> Enum.map(&parse_quantity/1)
      |> Enum.into(%{})
    }
  end

  defp parse_quantity(string) do
    [amount_string, chemical] = String.split(string, " ")
    {chemical, String.to_integer(amount_string)}
  end
end
