defmodule DayEight do
  @doc ~S"""
    Day 8 solver

    Works on the sample input:
      iex> DayEight.partOne("b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10\n")
      1
  """

  def partOne(input) do
    parse(input)
    |> Enum.reduce(%{registers: Map.new(), maximum: 0}, &execute/2)
    |> IO.inspect()
    |> Map.get(:registers)
    |> Map.values()
    |> Enum.max()
  end

  defp execute(%{command: command, conditional: conditional}, context) do
    if truthy?(conditional, context) do
      execute(command, context)
    else
      context
    end
  end

  defp execute(%{register: register, operation: operation, amount: amount}, %{registers: registers, maximum: maximum}) do
    register_value = fetch(register, registers)

    new_value = case operation do
      "inc" -> register_value + amount
      "dec" -> register_value - amount
      _ -> raise "Invalid operation encountered: #{operation}"
    end

    new_maximum = if new_value > maximum do
      new_value
    else
      maximum
    end

    %{registers: Map.put(registers, register, new_value), maximum: new_maximum}
  end

  defp truthy?(%{register: register, comparator: comparator, value: value}, %{registers: registers}) do
    register_value = fetch(register, registers)

    case comparator do
      ">" -> register_value > value
      ">=" -> register_value >= value
      "<" -> register_value < value
      "<=" -> register_value <= value
      "==" -> register_value == value
      "!=" -> register_value != value
      _ -> raise "Invalid comparator encountered: #{comparator}"
    end
  end

  defp fetch(register, registers) do
    Map.get(registers, register, 0)
  end

  defp parse(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parseInstruction/1)
  end

  defp parseInstruction(line) do
    [commandString, conditionalString] = String.split(line, " if ")

    %{
      command: parseCommand(commandString),
      conditional: parseConditional(conditionalString)
    }
  end

  defp parseCommand(string) do
    [register, operation, amount] = String.split(string)

    %{
      register: register,
      operation: operation,
      amount: String.to_integer(amount)
    }
  end

  defp parseConditional(string) do
    [register, comparator, value] = String.split(string)

    %{
      register: register,
      comparator: comparator,
      value: String.to_integer(value)
    }
  end
end
