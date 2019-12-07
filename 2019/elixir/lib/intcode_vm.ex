defmodule IntcodeVM do
  def run_for_output(intcode, inputs \\ []) do
    intcode
    |> run(inputs)
    |> Map.get(:outputs)
    |> List.first()
  end

  def run(intcode, inputs \\ [], ip \\ 0) do
    run(intcode, ip, inputs, [])
  end

  ## PRIVATE FUNCTIONS ##

  defp run(intcode, ip, inputs, outputs) do
    instruction = parse_instruction(intcode, ip)

    case instruction.operation do
      :add ->
        a = get_value(intcode, param(instruction, 0))
        b = get_value(intcode, param(instruction, 1))

        intcode
        |> set_value(a + b, param(instruction, 2))
        |> run(ip + instruction.size, inputs, outputs)

      :mul ->
        a = get_value(intcode, param(instruction, 0))
        b = get_value(intcode, param(instruction, 1))

        intcode
        |> set_value(a * b, param(instruction, 2))
        |> run(ip + instruction.size, inputs, outputs)

      :in ->
        case inputs do
          [] ->
            %{
              intcode: intcode,
              outputs: outputs,
              state: :waiting_for_input,
              ip: ip
            }

          [input | inputs] ->
            intcode
            |> set_value(input, param(instruction, 0))
            |> run(ip + instruction.size, inputs, outputs)
        end

      :out ->
        output = get_value(intcode, param(instruction, 0))
        run(intcode, ip + instruction.size, inputs, [output | outputs])

      :jit ->
        if get_value(intcode, param(instruction, 0)) != 0 do
          new_ip = get_value(intcode, param(instruction, 1))
          run(intcode, new_ip, inputs, outputs)
        else
          run(intcode, ip + instruction.size, inputs, outputs)
        end

      :jif ->
        if get_value(intcode, param(instruction, 0)) == 0 do
          new_ip = get_value(intcode, param(instruction, 1))
          run(intcode, new_ip, inputs, outputs)
        else
          run(intcode, ip + instruction.size, inputs, outputs)
        end

      :lt ->
        if get_value(intcode, param(instruction, 0)) <
             get_value(intcode, param(instruction, 1)) do
          set_value(intcode, 1, param(instruction, 2))
        else
          set_value(intcode, 0, param(instruction, 2))
        end
        |> run(ip + instruction.size, inputs, outputs)

      :eq ->
        if get_value(intcode, param(instruction, 0)) ==
             get_value(intcode, param(instruction, 1)) do
          set_value(intcode, 1, param(instruction, 2))
        else
          set_value(intcode, 0, param(instruction, 2))
        end
        |> run(ip + instruction.size, inputs, outputs)

      :exit ->
        %{intcode: intcode, outputs: outputs, state: :done}
    end
  end

  defp param(instruction, index) do
    Enum.at(instruction.params, index)
  end

  defp get_value(intcode, {:position, position}), do: Enum.at(intcode, position)
  defp get_value(_intcode, {:immediate, value}), do: value

  defp set_value(intcode, value, {:position, position}) do
    List.replace_at(intcode, position, value)
  end

  defp parse_instruction(intcode, position) do
    instruction = Enum.at(intcode, position)

    {param_modes_string, opcode_string} =
      instruction
      |> Integer.to_string()
      |> String.split_at(-2)

    opcode = String.to_integer(opcode_string)
    {operation, param_count} = translate_opcode(opcode)

    param_modes =
      param_modes_string
      |> String.reverse()
      |> String.graphemes()
      |> Enum.map(&String.to_integer/1)

    params =
      intcode
      |> Enum.slice(position + 1, param_count)
      |> Enum.with_index()
      |> Enum.map(fn {param, index} ->
        case Enum.at(param_modes, index, 0) do
          0 -> {:position, param}
          1 -> {:immediate, param}
        end
      end)

    %{
      operation: operation,
      params: params,
      size: param_count + 1
    }
  end

  defp translate_opcode(1), do: {:add, 3}
  defp translate_opcode(2), do: {:mul, 3}
  defp translate_opcode(3), do: {:in, 1}
  defp translate_opcode(4), do: {:out, 1}
  defp translate_opcode(5), do: {:jit, 2}
  defp translate_opcode(6), do: {:jif, 2}
  defp translate_opcode(7), do: {:lt, 3}
  defp translate_opcode(8), do: {:eq, 3}
  defp translate_opcode(99), do: {:exit, 0}
  defp translate_opcode(opcode), do: raise("invalid opcode: #{opcode}")
end
