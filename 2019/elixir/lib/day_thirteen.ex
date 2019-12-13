defmodule DayThirteen do
  @screen_width 41
  @screen_height 24

  defmodule State do
    defstruct screen: %{},
              paddle_position: nil,
              ball_position: nil,
              previous_ball_position: nil,
              score: nil,
              configuration: nil,
              steps: 0,
              input: 0,
              visualization: false
  end

  @doc """
    Part 1
  """
  def part_one(input) do
    %State{configuration: Intcode.new(input)}
    |> step()
    |> Map.get(:screen)
    |> Enum.count(fn {_location, tile} -> tile == 2 end)
  end

  @doc """
    Part 2
  """
  def part_two(input, visualization \\ false) do
    if(visualization, do: IO.write(IO.ANSI.clear()))

    %State{
      visualization: visualization,
      configuration:
        input
        |> String.replace_prefix("1", "2")
        |> Intcode.new()
    }
    |> step()
    |> Map.get(:score)
  end

  ## PRIVATE FUNCTIONS ##

  defp step(%State{configuration: %{state: :done}} = state) do
    state
  end

  defp step(%State{} = state) do
    state
    |> next_state()
    |> display()
    |> step()
  end

  defp next_input(%State{
         ball_position: {x, y},
         previous_ball_position: {xx, yy},
         paddle_position: {px, py}
       }) do
    desired_x =
      if y >= 15 do
        m = (y - yy) / (x - xx)
        m = m * if(y < yy, do: -1, else: 1)
        constrain((py - y + m * x) / m)
      else
        x
      end

    cond do
      desired_x == px -> if(x > px, do: 1, else: -1)
      desired_x > px -> 1
      desired_x < px -> -1
    end
  end

  defp next_input(_state), do: 0

  defp constrain(x) when x < 0, do: abs(x)

  defp constrain(x) when x > @screen_width do
    @screen_width - (x - @screen_width)
  end

  defp constrain(x), do: x

  defp next_state(%State{configuration: configuration} = state) do
    input = next_input(state)

    {outputs, configuration} =
      configuration
      |> Intcode.put_inputs([input])
      |> Intcode.run()
      |> Intcode.pop_outputs()

    outputs
    |> Enum.chunk_every(3)
    |> Enum.reduce(%{state | configuration: configuration, input: input}, fn
      [-1, 0, score], state ->
        put_in(state, [Access.key(:score)], score)

      [x, y, 3], state ->
        put_in(state, [Access.key(:paddle_position)], {x, y})

      [x, y, 4], state ->
        state
        |> put_in([Access.key(:previous_ball_position)], state.ball_position)
        |> put_in([Access.key(:ball_position)], {x, y})

      [x, y, tile], state ->
        put_in(state, [Access.key(:screen), {x, y}], tile)
    end)
    |> Map.update(:steps, 1, &(&1 + 1))
  end

  defp display(%State{visualization: false} = state), do: state

  defp display(%State{visualization: true} = state) do
    IO.puts(
      "#{IO.ANSI.cursor(0, 0)}STEP:  #{state.steps |> with_leading_zeros(6)}  SCORE:  #{
        state.score |> with_leading_zeros(6)
      }"
    )

    IO.puts(
      "BALL: #{format_point(state.ball_position)}  PADDLE: #{
        format_point(state.paddle_position)
      }\n"
    )

    for y <- 0..(@screen_height - 1) do
      IO.write("#{with_leading_zeros(y, 2)} ")

      for x <- 0..(@screen_width - 1) do
        cond do
          {x, y} == state.ball_position ->
            "●"

          {x, y} == state.paddle_position ->
            "⏤"

          true ->
            case Map.get(state.screen, {x, y}) do
              0 -> " "
              1 -> "█"
              2 -> "🀫"
              nil -> "ⓧ"
            end
        end
        |> IO.write()
      end

      IO.write("\n")
    end

    IO.write("   ⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧⌧\n")

    state
  end

  defp format_point({x, y}) do
    "(#{with_leading_zeros(x, 2)},#{with_leading_zeros(y, 2)})"
  end

  defp with_leading_zeros(integer, size) do
    integer
    |> Integer.to_string()
    |> String.pad_leading(size, "0")
  end
end
