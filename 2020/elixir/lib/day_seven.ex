defmodule DaySeven do
  @doc """
    iex> DaySeven.part_one("light red bags contain 1 bright white bag, 2 muted yellow bags.\\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\\nbright white bags contain 1 shiny gold bag.\\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\\nfaded blue bags contain no other bags.\\ndotted black bags contain no other bags.\\n")
    4
  """
  def part_one(input) do
    input
    |> parse()
    |> bags_that_can_contain("shiny gold")
    |> Enum.count()
  end

  @doc """
    iex> DaySeven.part_two("light red bags contain 1 bright white bag, 2 muted yellow bags.\\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\\nbright white bags contain 1 shiny gold bag.\\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\\nfaded blue bags contain no other bags.\\ndotted black bags contain no other bags.\\n")
    32
  """
  def part_two(input) do
    rules = parse(input)
    required_bag_count(rules, "shiny gold")
  end

  ## PRIVATE_FUNCTIONS

  defp required_bag_count(rules, target_bag) do
    rules
    |> Map.get(target_bag)
    |> Enum.reduce(0, fn {bag, count}, total ->
      total + count + count * required_bag_count(rules, bag)
    end)
  end

  defp bags_that_can_contain(rules, target_bag) do
    bags =
      Enum.reduce(rules, MapSet.new(), fn {bag, contents}, bags ->
        if Map.has_key?(contents, target_bag) do
          MapSet.put(bags, bag)
        else
          bags
        end
      end)

    MapSet.union(
      bags,
      MapSet.new(Enum.flat_map(bags, &bags_that_can_contain(rules, &1)))
    )
  end

  defp parse(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(fn line ->
      [bag, contents_text] = String.split(line, " bags contain ", parts: 2, trim: true)

      contents =
        contents_text
        |> String.split([", ", "."], trim: true)
        |> Enum.reject(&(&1 == "no other bags"))
        |> Enum.map(fn bag_and_count ->
          {count, bag_text} = Integer.parse(bag_and_count)

          bag =
            bag_text
            |> String.trim()
            |> String.trim_trailing(" bags")
            |> String.trim_trailing(" bag")

          {bag, count}
        end)
        |> Enum.into(%{})

      {bag, contents}
    end)
    |> Enum.into(%{})
  end
end
