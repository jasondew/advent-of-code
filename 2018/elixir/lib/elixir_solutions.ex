defmodule ElixirSolutions do
  def day_one do
    input = day_input("01")
    IO.puts("Part 1: #{ElixirSolutions.DayOne.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayOne.part_two(input)}")
  end

  def day_two do
    input = day_input("02")
    IO.puts("Part 1: #{ElixirSolutions.DayTwo.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayTwo.part_two(input)}")
  end

  def day_three do
    input = day_input("03")
    IO.puts("Part 1: #{ElixirSolutions.DayThree.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayThree.part_two(input)}")
  end

  def day_four do
    input = day_input("04")
    IO.puts("Part 1: #{ElixirSolutions.DayFour.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayFour.part_two(input)}")
  end

  def day_five do
    input = day_input("05")
    IO.puts("Part 1: #{ElixirSolutions.DayFive.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayFive.part_two(input)}")
  end

  def day_six do
    input = day_input("06")
    IO.puts("Part 1: #{ElixirSolutions.DaySix.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DaySix.part_two(input)}")
  end

  def day_seven do
    input = day_input("07")
    IO.puts("Part 1: #{ElixirSolutions.DaySeven.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DaySeven.part_two(input)}")
  end

  def day_eight do
    input = day_input("08")
    IO.puts("Part 1: #{ElixirSolutions.DayEight.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayEight.part_two(input)}")
  end

  def day_nine do
    input = day_input("09")
    IO.puts("Part 1: #{ElixirSolutions.DayNine.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayNine.part_two(input)}")
  end

  def day_ten do
    ElixirSolutions.DayTen.parts_one_and_two(day_input("10"), 11_000, print_min: true)
  end

  def day_eleven do
    input = day_input("11")
    IO.puts("Part 1: #{inspect(ElixirSolutions.DayEleven.part_one(input))}")
    IO.puts("Part 2: #{inspect(ElixirSolutions.DayEleven.part_two(input))}")
  end

  def day_twelve do
    input = day_input("12")
    IO.puts("Part 1: #{inspect(ElixirSolutions.DayTwelve.part_one(input))}")
    IO.puts("Part 2: #{inspect(ElixirSolutions.DayTwelve.part_two())}")
  end

  def day_thirteen do
    input = day_input("13", false)
    IO.puts("Part 1: #{inspect(ElixirSolutions.DayThirteen.part_one(input))}")
    IO.puts("Part 2: #{inspect(ElixirSolutions.DayThirteen.part_two(input))}")
  end

  def day_fourteen do
    input = day_input("14")
    #    IO.puts("Part 1: #{inspect(ElixirSolutions.DayFourteen.part_one(input))}")
    IO.puts("Part 2: #{inspect(ElixirSolutions.DayFourteen.part_two(input))}")
  end

  def day_input(day, trim \\ true) do
    {:ok, input} = File.read("../inputs/#{day}")
    if trim, do: String.trim(input), else: input
  end
end
