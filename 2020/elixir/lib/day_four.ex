defmodule DayFour do
  @doc """
    Part 1

    iex> DayFour.part_one("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\\nbyr:1937 iyr:2017 cid:147 hgt:183cm\\n\\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\\nhcl:#cfa07d byr:1929\\n\\nhcl:#ae17e1 iyr:2013\\neyr:2024\\necl:brn pid:760753108 byr:1931\\nhgt:179cm\\n\\nhcl:#cfa07d eyr:2025 pid:166559648\\niyr:2011 ecl:brn hgt:59in\\n")
    2
  """
  def part_one(input) do
    input
    |> parse_passports()
    |> Enum.count(&present?/1)
  end

  @doc """
    Part 2

    iex> DayFour.part_two("eyr:1972 cid:100\\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\\n\\niyr:2019\\nhcl:#602927 eyr:1967 hgt:170cm\\necl:grn pid:012533040 byr:1946\\n\\nhcl:dab227 iyr:2012\\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\\n\\nhgt:59cm ecl:zzz\\neyr:2038 hcl:74454a iyr:2023\\npid:3556412378 byr:2007\\n\\npid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\\nhcl:#623a2f\\n\\neyr:2029 ecl:blu cid:129 byr:1989\\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\\n\\nhcl:#888785\\nhgt:164cm byr:2001 iyr:2015 cid:88\\npid:545766238 ecl:hzl\\neyr:2022\\n\\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 pid:093154719")
    3
  """
  def part_two(input) do
    input
    |> parse_passports()
    |> Enum.count(fn passport ->
      present?(passport) and valid?(passport)
    end)
  end

  ## PRIVATE FUNCTIONS

  defp valid?(passport) do
    Enum.all?(passport, fn {field, value} -> valid?(field, value) end)
  end

  defp valid?("byr", value), do: integer_between(value, 1920, 2020)
  defp valid?("iyr", value), do: integer_between(value, 2010, 2020)
  defp valid?("eyr", value), do: integer_between(value, 2020, 2030)

  defp valid?("hgt", value) do
    case Integer.parse(value) do
      {centimeters, "cm"} -> between(centimeters, 150, 193)
      {inches, "in"} -> between(inches, 59, 76)
      _ -> false
    end
  end

  defp valid?("hcl", "#" <> value) do
    case Integer.parse(value, 16) do
      {color, ""} -> color <= 16_777_215
      _ -> false
    end
  end

  defp valid?("ecl", value) when value in ~w[amb blu brn gry grn hzl oth], do: true

  defp valid?("pid", value) do
    case Integer.parse(value) do
      {_pid, ""} -> String.length(value) == 9
      _ -> false
    end
  end

  defp valid?("cid", _value), do: true

  defp valid?(_, _), do: false

  defp present?(%{
         "byr" => _,
         "iyr" => _,
         "eyr" => _,
         "hgt" => _,
         "hcl" => _,
         "ecl" => _,
         "pid" => _
       }) do
    true
  end

  defp present?(_), do: false

  defp integer_between(string, from, to) do
    case Integer.parse(string) do
      {value, ""} -> between(value, from, to)
      _ -> false
    end
  end

  defp between(value, from, to), do: value >= from and value <= to

  defp parse_passports(input) do
    input
    |> String.trim()
    |> String.split("\n\n")
    |> Enum.map(fn lines ->
      lines
      |> String.split([" ", "\n"])
      |> Enum.map(fn key_value_pair ->
        key_value_pair
        |> String.split(":", parts: 2, trim: true)
        |> List.to_tuple()
      end)
      |> Enum.into(%{})
    end)
  end
end
