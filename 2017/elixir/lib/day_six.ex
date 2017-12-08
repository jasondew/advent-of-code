defmodule DaySix do
  @doc ~S"""
  Part 1 solver

  Given 4 memory banks and an initial configuration of 0, 2, 7, 0, a loop
  is detected in 5 steps.

    iex> DaySix.partOne("0 2 7 0")
    5
  """
  def partOne(input) do
    blocks =
      input
      |> String.split()
      |> Enum.map(&String.to_integer/1)

    run_until_loop_detected(blocks, 0, [])
  end

  defp run_until_loop_detected(_blocks, _cycles, _previous_states) do
#    if previous_index = Enum.find_index(blocks, &(&1 == previous_states)) do
#      IO.puts "Loop of size #{cycles - previous_index} detected at cycle #{cycles}!"
#    else
#      updated_blocks = blocks.dup
#      target_index = blocks.find_index(blocks.max)
#      blocks_to_reallocate, updated_blocks[target_index] = updated_blocks[target_index], 0
#
#      blocks_to_reallocate.times do
#        target_index = (target_index + 1) % updated_blocks.size
#        updated_blocks[target_index] += 1
#      end
#
#      run_until_loop_detected(
#    end
  end
end
