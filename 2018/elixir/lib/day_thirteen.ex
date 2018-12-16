defmodule ElixirSolutions.DayThirteen do
  defmodule Mine do
    defstruct [:tracks, :intersections, :carts, :next_track_id, :next_cart_id]

    def new() do
      %__MODULE__{
        tracks: %{},
        intersections: %{},
        carts: %{},
        next_track_id: 1,
        next_cart_id: 1
      }
    end
  end

  defmodule Track do
    defstruct [:id, :top_left, :width, :height]

    def new(id, position), do: %__MODULE__{id: id, top_left: position}

    def beginning_or_ending_at_x?(track, x) do
      known_x_positions =
        if track.width do
          [track.top_left.x, top_right_corner(track).x]
        else
          [track.top_left.x]
        end

      Enum.member?(known_x_positions, x)
    end

    def next_position_and_direction(track, position, direction) do
      new_position =
        case direction do
          :north -> %{position | y: position.y - 1}
          :south -> %{position | y: position.y + 1}
          :east -> %{position | x: position.x + 1}
          :west -> %{position | x: position.x - 1}
        end

      new_direction =
        case corner_or_side(track, new_position) do
          :side ->
            direction

          :top_left ->
            case direction do
              :north -> :east
              :west -> :south
            end

          :top_right ->
            case direction do
              :north -> :west
              :east -> :south
            end

          :bottom_left ->
            case direction do
              :south -> :east
              :west -> :north
            end

          :bottom_right ->
            case direction do
              :south -> :west
              :east -> :north
            end
        end

      {new_position, new_direction}
    end

    defp corner_or_side(track, position) do
      cond do
        position == top_left_corner(track) -> :top_left
        position == top_right_corner(track) -> :top_right
        position == bottom_left_corner(track) -> :bottom_left
        position == bottom_right_corner(track) -> :bottom_right
        true -> :side
      end
    end

    def top_left_corner(track), do: track.top_left
    def top_right_corner(track), do: %{track.top_left | x: track.top_left.x + track.width - 1}
    def bottom_left_corner(track), do: %{track.top_left | y: track.top_left.y + track.height - 1}

    def bottom_right_corner(track),
      do: %{
        track.top_left
        | x: track.top_left.x + track.width - 1,
          y: track.top_left.y + track.height - 1
      }
  end

  defmodule Cart do
    defstruct [:id, :track_id, :position, :direction, :next_turn, :crashed]

    def new(id, track_id, position, direction) do
      %__MODULE__{
        id: id,
        track_id: track_id,
        position: position,
        direction: direction,
        next_turn: :left,
        crashed: false
      }
    end
  end

  defmodule Position do
    defstruct [:x, :y]
  end

  @doc ~S"""
    iex> ElixirSolutions.DayThirteen.part_one([
    ...>'/->-\\        ',
    ...>'|   |  /----\\',
    ...>'| /-+--+-\\  |',
    ...>'| | |  | v  |',
    ...>'\\-+-/  \\-+--/',
    ...>'  \\------/   ',
    ...>] |> Enum.join("\n")
    ...>)
    %ElixirSolutions.DayThirteen.Position{x: 7, y: 3}
  """
  def part_one(input) do
    Enum.reduce_while(1..1_000, parse(input), fn _turn, mine ->
      crashed_carts = Enum.filter(Map.values(mine.carts), &(&1.crashed == true))

      if Enum.count(crashed_carts) > 1 do
        {:halt, hd(crashed_carts).position}
      else
        {:cont, step(mine)}
      end
    end)
    |> case do
      %Position{} = position -> position
      %Mine{} -> :not_found
    end
  end

  def part_two(input) do
    Enum.reduce_while(1..25_000, parse(input), fn _turn, mine ->
      uncrashed_carts = Enum.filter(Map.values(mine.carts), &(&1.crashed == false))

      if Enum.count(uncrashed_carts) == 1 do
        {:halt, hd(uncrashed_carts).position}
      else
        {:cont, step(mine)}
      end
    end)
    |> case do
      %Position{} = position -> position
      %Mine{} -> :not_found
    end
  end

  def step(mine) do
    sorted_uncrashed_carts =
      mine.carts
      |> Map.values()
      |> Enum.filter(&(&1.crashed == false))
      |> Enum.sort_by(&{&1.position.y, &1.position.x})

    #    IO.puts(
    #      "processing carts #{
    #        sorted_uncrashed_carts
    #        |> Enum.map(fn cart ->
    #          "#{to_string(cart.id)}-(#{to_string(cart.position.x)},#{to_string(cart.position.y)})"
    #        end)
    #        |> Enum.join(", ")
    #      }"
    #    )

    Enum.reduce(sorted_uncrashed_carts, mine, fn cart, mine ->
      if still_alive?(cart, mine) do
        cart
        |> step_cart(mine)
        |> update_carts_with_crash_detection(mine)
      else
        mine
      end
    end)
  end

  def step_cart(cart, mine) do
    case Map.get(mine.intersections, cart.position) do
      nil ->
        move_along_track(mine, cart)

      intersecting_track_ids ->
        intersecting_track_id = Enum.find(intersecting_track_ids, &(&1 != cart.track_id))

        if cart.next_turn == :straight do
          move_along_track(mine, %{cart | next_turn: next_turn(cart.next_turn)})
        else
          move_along_track(mine, %{
            cart
            | next_turn: next_turn(cart.next_turn),
              track_id: intersecting_track_id,
              direction: next_direction(cart.direction, cart.next_turn)
          })
        end
    end
  end

  def still_alive?(cart, mine) do
    get_cart(cart.id, mine).crashed == false
  end

  def get_cart(id, mine) do
    mine
    |> Map.get(:carts)
    |> Map.get(id)
  end

  def update_carts_with_crash_detection(cart, mine) do
    uncrashed_carts =
      mine.carts
      |> Map.values()
      |> Enum.filter(&(&1.crashed == false))

    case Enum.find(uncrashed_carts, &(&1.position == cart.position)) do
      nil ->
        update_cart(cart, mine)

      crashed_cart ->
        #        IO.inspect("carts #{cart.id} and #{crashed_cart.id} crashed!")
        update_cart(%{cart | crashed: true}, update_cart(%{crashed_cart | crashed: true}, mine))
    end
  end

  def update_cart(cart, mine) do
    Map.update!(mine, :carts, &Map.put(&1, cart.id, cart))
  end

  def next_turn(:left), do: :straight
  def next_turn(:straight), do: :right
  def next_turn(:right), do: :left

  def next_direction(:north, :left), do: :west
  def next_direction(:north, :right), do: :east
  def next_direction(:south, :left), do: :east
  def next_direction(:south, :right), do: :west
  def next_direction(:east, :left), do: :north
  def next_direction(:east, :right), do: :south
  def next_direction(:west, :left), do: :south
  def next_direction(:west, :right), do: :north

  def move_along_track(mine, cart) do
    track = Map.get(mine.tracks, cart.track_id)

    {new_position, new_direction} =
      Track.next_position_and_direction(track, cart.position, cart.direction)

    %{cart | position: new_position, direction: new_direction}
  end

  @doc ~S"""
    iex> ElixirSolutions.DayThirteen.parse([
    ...>' /->-\\        ',
    ...>' |   |  /----\\',
    ...>' | /-+--+-\\  |',
    ...>' | | |  | v  |',
    ...>' \\-+-/  \\-+--/',
    ...>'   \\------/   ',
    ...>] |> Enum.join("\n")
    ...>)
    %ElixirSolutions.DayThirteen.Mine{
      carts: %{
        2 => %ElixirSolutions.DayThirteen.Cart{
          direction: :south,
          id: 2,
          position: %ElixirSolutions.DayThirteen.Position{x: 10, y: 3},
          track_id: 3,
          next_turn: :left,
          crashed: false
        },
        1 => %ElixirSolutions.DayThirteen.Cart{
          direction: :east,
          id: 1,
          position: %ElixirSolutions.DayThirteen.Position{x: 3, y: 0},
          track_id: 1,
          next_turn: :left,
          crashed: false
        }
      },
      intersections: %{
        %ElixirSolutions.DayThirteen.Position{x:  3, y: 4} => [1, 3],
        %ElixirSolutions.DayThirteen.Position{x:  5, y: 2} => [3, 1],
        %ElixirSolutions.DayThirteen.Position{x:  8, y: 2} => [3, 2],
        %ElixirSolutions.DayThirteen.Position{x: 10, y: 4} => [2, 3]
      },
      next_cart_id: 3,
      next_track_id: 4,
      tracks: %{
        1 => %ElixirSolutions.DayThirteen.Track{
          height: 5,
          id: 1,
          top_left: %ElixirSolutions.DayThirteen.Position{x: 1, y: 0},
          width: 5
        },
        2 => %ElixirSolutions.DayThirteen.Track{
          height: 4,
          id: 2,
          top_left: %ElixirSolutions.DayThirteen.Position{x: 8, y: 1},
          width: 6
        },
        3 => %ElixirSolutions.DayThirteen.Track{
          height: 4,
          id: 3,
          top_left: %ElixirSolutions.DayThirteen.Position{x: 3, y: 2},
          width: 8
        }
      }
    }
  """
  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.with_index()
    |> Enum.reduce(Mine.new(), fn {line, y}, mine ->
      line
      |> String.codepoints()
      |> Enum.with_index()
      |> Enum.reduce({mine, :clear}, fn {character, x}, {mine, state} ->
        position = %Position{x: x, y: y}

        case {state, character} do
          {:clear, "/"} ->
            {track_start(mine, position), {:on_track, mine.next_track_id}}

          {:clear, "|"} ->
            {mine, :clear}

          {:clear, " "} ->
            {mine, :clear}

          {:clear, "\\"} ->
            {mine, {:on_track, find_nearest_track_id_above(mine, position)}}

          {:clear, "^"} ->
            {add_cart(mine, find_nearest_track_id_above(mine, position), position, :north),
             :clear}

          {:clear, "v"} ->
            {add_cart(mine, find_nearest_track_id_above(mine, position), position, :south),
             :clear}

          {{:on_track, id}, "\\"} ->
            {track_turn_downward(mine, id, position), :clear}

          {{:on_track, _id}, "-"} ->
            {mine, state}

          {{:on_track, id}, "/"} ->
            {track_complete(mine, id, position), :clear}

          {{:on_track, id}, "+"} ->
            {add_intersection(mine, id, position), state}

          {{:on_track, id}, ">"} ->
            {add_cart(mine, id, position, :east), state}

          {{:on_track, id}, "<"} ->
            {add_cart(mine, id, position, :west), state}
        end
      end)
      |> elem(0)
    end)
  end

  defp add_intersection(mine, track_id, position) do
    intersecting_track_id = find_nearest_track_id_above(mine, position)

    Map.update!(mine, :intersections, fn intersections ->
      Map.put(intersections, position, [track_id, intersecting_track_id])
    end)
  end

  defp add_cart(mine, track_id, position, direction) do
    mine
    |> Map.update!(:carts, fn carts ->
      Map.put(
        carts,
        mine.next_cart_id,
        Cart.new(mine.next_cart_id, track_id, position, direction)
      )
    end)
    |> Map.put(:next_cart_id, mine.next_cart_id + 1)
  end

  defp find_nearest_track_id_above(mine, position) do
    mine.tracks
    |> Enum.filter(fn {_id, track} -> Track.beginning_or_ending_at_x?(track, position.x) end)
    |> Enum.map(fn {id, _track} -> id end)
    |> Enum.max()
  end

  defp track_start(mine, position) do
    mine
    |> Map.update!(:tracks, fn tracks ->
      Map.put(tracks, mine.next_track_id, Track.new(mine.next_track_id, position))
    end)
    |> Map.put(:next_track_id, mine.next_track_id + 1)
  end

  defp track_turn_downward(mine, id, position) do
    Map.update!(mine, :tracks, fn tracks ->
      Map.update!(tracks, id, fn track ->
        Map.put(track, :width, position.x - track.top_left.x + 1)
      end)
    end)
  end

  defp track_complete(mine, id, position) do
    Map.update!(mine, :tracks, fn tracks ->
      Map.update!(tracks, id, fn track ->
        Map.put(track, :height, position.y - track.top_left.y + 1)
      end)
    end)
  end
end
