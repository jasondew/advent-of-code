defmodule DayEight do
  @max_steps 10_000

  defmodule State do
    defstruct acc: 0, pc: 0, instructions: [], visited: [], steps: 0, mode: :initialized

    def new(input) do
      instructions =
        input
        |> String.trim()
        |> String.split("\n")
        |> Enum.map(fn line ->
          [op, arg_string] = String.split(line, " ", parts: 2, trim: true)
          {arg, ""} = Integer.parse(arg_string)

          {op, arg}
        end)

      %__MODULE__{instructions: instructions}
    end
  end

  @doc ~S[
    iex> """
    ...> nop +0
    ...> acc +1
    ...> jmp +4
    ...> acc +3
    ...> jmp -3
    ...> acc -99
    ...> acc +1
    ...> jmp -4
    ...> acc +6
    ...> """ |> DayEight.part_one()
    5
  ]
  def part_one(input) do
    input
    |> State.new()
    |> run_until_loop_detected()
    |> Map.get(:acc)
  end

  @doc ~S[
    iex> """
    ...> nop +0
    ...> acc +1
    ...> jmp +4
    ...> acc +3
    ...> jmp -3
    ...> acc -99
    ...> acc +1
    ...> jmp -4
    ...> acc +6
    ...> """ |> DayEight.part_two()
    8
  ]
  def part_two(input) do
    state = State.new(input)

    state.instructions
    |> Enum.with_index()
    |> Enum.filter(fn {{op, arg}, _index} ->
      op in ~w[nop jmp] and {op, arg} != {"nop", 0}
    end)
    |> Enum.find_value(fn {{op, _arg}, index} ->
      final_state =
        state
        |> swap_instruction(op, index)
        |> run_until_loop_detected()

      if(final_state.mode == :finished, do: final_state.acc)
    end)
  end

  ## PRIVATE FUNCTIONS

  defp swap_instruction(state, op, index) do
    new_op =
      case op do
        "jmp" -> "nop"
        "nop" -> "jmp"
      end

    %{
      state
      | instructions:
          List.update_at(state.instructions, index, fn {_op, arg} ->
            {new_op, arg}
          end)
    }
  end

  defp run_until_loop_detected(state, max_steps \\ @max_steps) do
    if state.pc in state.visited || state.mode == :finished || state.steps >= max_steps do
      state
    else
      state
      |> step()
      |> run_until_loop_detected()
    end
  end

  defp step(state) do
    case Enum.at(state.instructions, state.pc) do
      nil -> finish(state)
      instruction -> execute(instruction, state)
    end
  end

  defp execute({"acc", value}, state) do
    state
    |> update_acc(&(&1 + value))
    |> commit()
  end

  defp execute({"jmp", offset}, state), do: commit(state, offset)
  defp execute({"nop", _}, state), do: commit(state)

  defp update_acc(%State{acc: acc} = state, fun) do
    %{state | acc: fun.(acc)}
  end

  defp commit(%State{pc: pc, visited: visited, steps: steps} = state, offset \\ 1) do
    %{state | pc: pc + offset, visited: [pc | visited], steps: steps + 1, mode: :running}
  end

  defp finish(%State{} = state), do: %{state | mode: :finished}
end
