defmodule Intcode do
  defmodule Configuration do
    defstruct memory: [],
              size: 0,
              ip: 0,
              bp: 0,
              inputs: [],
              outputs: [],
              state: :running,
              iterations: 0
  end

  def new(memory, inputs \\ [], ip \\ 0) do
    %Configuration{
      memory: memory,
      size: Enum.count(memory),
      inputs: inputs,
      ip: ip
    }
  end

  def run_for_output(memory, inputs \\ []) do
    memory
    |> run(inputs)
    |> Map.get(:outputs)
    |> List.first()
  end

  def run(memory, inputs, ip \\ 0) do
    memory
    |> new(inputs, ip)
    |> run()
  end

  def run(%Configuration{ip: ip, bp: bp} = configuration) do
    configuration = Map.update(configuration, :iterations, 1, &(&1 + 1))
    instruction = parse_instruction(configuration)

    case instruction.operation do
      :add ->
        a = get_param_value(configuration, instruction, 0)
        b = get_param_value(configuration, instruction, 1)

        configuration
        |> set_value_at_param(a + b, instruction, 2)
        |> Map.put(:ip, ip + instruction.size)
        |> run()

      :mul ->
        a = get_param_value(configuration, instruction, 0)
        b = get_param_value(configuration, instruction, 1)

        configuration
        |> set_value_at_param(a * b, instruction, 2)
        |> Map.put(:ip, ip + instruction.size)
        |> run()

      :in ->
        case configuration.inputs do
          [] ->
            %{configuration | state: :waiting_for_input}

          [input | inputs] ->
            configuration
            |> set_value_at_param(input, instruction, 0)
            |> Map.put(:ip, ip + instruction.size)
            |> Map.put(:inputs, inputs)
            |> run()
        end

      :out ->
        output = get_param_value(configuration, instruction, 0)

        run(%{
          configuration
          | ip: ip + instruction.size,
            outputs: [output | configuration.outputs]
        })

      :jit ->
        if get_param_value(configuration, instruction, 0) != 0 do
          run(%{
            configuration
            | ip: get_param_value(configuration, instruction, 1)
          })
        else
          run(%{configuration | ip: ip + instruction.size})
        end

      :jif ->
        if get_param_value(configuration, instruction, 0) == 0 do
          run(%{
            configuration
            | ip: get_param_value(configuration, instruction, 1)
          })
        else
          run(%{configuration | ip: ip + instruction.size})
        end

      :lt ->
        if get_param_value(configuration, instruction, 0) <
             get_param_value(configuration, instruction, 1) do
          configuration
          |> set_value_at_param(1, instruction, 2)
          |> Map.put(:ip, ip + instruction.size)
          |> run()
        else
          configuration
          |> set_value_at_param(0, instruction, 2)
          |> Map.put(:ip, ip + instruction.size)
          |> run()
        end

      :eq ->
        if get_param_value(configuration, instruction, 0) ==
             get_param_value(configuration, instruction, 1) do
          configuration
          |> set_value_at_param(1, instruction, 2)
          |> Map.put(:ip, ip + instruction.size)
          |> run()
        else
          configuration
          |> set_value_at_param(0, instruction, 2)
          |> Map.put(:ip, ip + instruction.size)
          |> run()
        end

      :rbo ->
        configuration
        |> Map.put(:ip, ip + instruction.size)
        |> Map.put(:bp, bp + get_param_value(configuration, instruction, 0))
        |> run()

      :exit ->
        %{configuration | state: :done}
    end
  end

  ## PRIVATE FUNCTIONS ##

  defp get_param_value(
         %Configuration{bp: bp} = configuration,
         instruction,
         index
       ) do
    case Enum.at(instruction.params, index) do
      {:position, position} -> get_value(configuration, position)
      {:relative, offset} -> get_value(configuration, bp + offset)
      {:immediate, value} -> value
    end
  end

  def get_value(%Configuration{memory: memory, size: size}, location) do
    if location < size do
      Enum.at(memory, location)
    else
      0
    end
  end

  defp set_value_at_param(
         %Configuration{} = configuration,
         value,
         instruction,
         index
       ) do
    location =
      case Enum.at(instruction.params, index) do
        {:position, position} -> position
        {:relative, offset} -> configuration.bp + offset
      end

    configuration
    |> ensure_memory_space(location)
    |> Map.update(:memory, [], &List.replace_at(&1, location, value))
  end

  defp ensure_memory_space(configuration, location_requested) do
    if location_requested < configuration.size do
      configuration
    else
      extra_memory = for _ <- 1..10, do: 0

      configuration
      |> Map.put(:memory, Enum.concat(configuration.memory, extra_memory))
      |> Map.update(:size, 10, &(&1 + 10))
      |> ensure_memory_space(location_requested)
    end
  end

  defp parse_instruction(%Configuration{memory: memory, ip: ip}) do
    instruction = Enum.at(memory, ip)

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
      memory
      |> Enum.slice(ip + 1, param_count)
      |> Enum.with_index()
      |> Enum.map(fn {param, index} ->
        case Enum.at(param_modes, index, 0) do
          0 -> {:position, param}
          1 -> {:immediate, param}
          2 -> {:relative, param}
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
  defp translate_opcode(9), do: {:rbo, 1}
  defp translate_opcode(99), do: {:exit, 0}
  defp translate_opcode(opcode), do: raise("invalid opcode: #{opcode}")
end
