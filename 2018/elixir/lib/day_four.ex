defmodule ElixirSolutions.DayFour do
  defmodule Record do
    defstruct [:timestamp, :guard_id, :activity]
  end

  @doc ~s"""
    Part 1

    iex> ElixirSolutions.DayFour.part_one(
    ...>"[1518-11-01 00:00] Guard #10 begins shift
    ...>[1518-11-01 00:25] wakes up
    ...>[1518-11-01 00:55] wakes up
    ...>[1518-11-01 23:58] Guard #99 begins shift
    ...>[1518-11-02 00:40] falls asleep
    ...>[1518-11-03 00:29] wakes up
    ...>[1518-11-03 00:05] Guard #10 begins shift
    ...>[1518-11-03 00:24] falls asleep
    ...>[1518-11-05 00:03] Guard #99 begins shift
    ...>[1518-11-04 00:02] Guard #99 begins shift
    ...>[1518-11-01 00:05] falls asleep
    ...>[1518-11-02 00:50] wakes up
    ...>[1518-11-04 00:36] falls asleep
    ...>[1518-11-04 00:46] wakes up
    ...>[1518-11-05 00:45] falls asleep
    ...>[1518-11-01 00:30] falls asleep
    ...>[1518-11-05 00:55] wakes up")
    240
  """

  def part_one(input) do
    sleeps =
      input
      |> parse_records()
      |> sleeps_by_guard_id()

    sleepiest_guard_id = find_sleepiest_guard_id(sleeps)

    sleepiest_guard_id * most_common_sleeping_minute(sleeps[sleepiest_guard_id])
  end

  defp most_common_sleeping_minute(sleeps) do
    sleeps
    |> Enum.reduce(%{}, fn sleep, minutes ->
      Enum.reduce(
        sleep,
        minutes,
        fn minute, ms -> Map.update(ms, minute, 0, &(&1 + 1)) end
      )
    end)
    |> Enum.sort_by(fn {_minute, total} -> -total end)
    |> hd()
    |> elem(0)
  end

  defp find_sleepiest_guard_id(sleeps) do
    sleeps
    |> Enum.map(fn {guard_id, guard_sleeps} ->
      {guard_id, Enum.reduce(guard_sleeps, 0, &(Enum.count(&1) + &2))}
    end)
    |> Enum.sort_by(fn {_guard_id, minutes_slept} -> -minutes_slept end)
    |> hd()
    |> elem(0)
  end

  @doc ~S"""
    Part 2

    iex> ElixirSolutions.DayFour.part_two(
    ...>"[1518-11-01 00:00] Guard #10 begins shift
    ...>[1518-11-01 00:25] wakes up
    ...>[1518-11-01 00:55] wakes up
    ...>[1518-11-01 23:58] Guard #99 begins shift
    ...>[1518-11-02 00:40] falls asleep
    ...>[1518-11-03 00:29] wakes up
    ...>[1518-11-03 00:05] Guard #10 begins shift
    ...>[1518-11-03 00:24] falls asleep
    ...>[1518-11-05 00:03] Guard #99 begins shift
    ...>[1518-11-04 00:02] Guard #99 begins shift
    ...>[1518-11-01 00:05] falls asleep
    ...>[1518-11-02 00:50] wakes up
    ...>[1518-11-04 00:36] falls asleep
    ...>[1518-11-04 00:46] wakes up
    ...>[1518-11-05 00:45] falls asleep
    ...>[1518-11-01 00:30] falls asleep
    ...>[1518-11-05 00:55] wakes up")
    4455
  """

  def part_two(input) do
    input
    |> parse_records()
    |> sleeps_by_guard_id()
    |> Enum.map(fn {guard_id, sleeps} ->
      sleep_count_by_minute =
        sleeps
        |> Enum.flat_map(&Enum.to_list/1)
        |> Enum.sort()
        |> Enum.reduce(%{}, fn x, acc -> Map.update(acc, x, 1, &(&1 + 1)) end)

      {most_common_minute_slept, times_sleeping} =
        sleep_count_by_minute
        |> Enum.sort_by(fn {_minute, count} -> -count end)
        |> hd()

      {guard_id, most_common_minute_slept, times_sleeping}
    end)
    |> Enum.sort_by(fn {_guard_id, _most_common_minute_slept, times_sleeping} ->
      -times_sleeping
    end)
    |> hd()
    |> (fn {guard_id, most_common_minute_slept, _times_sleeping} ->
          guard_id * most_common_minute_slept
        end).()
  end

  defp sleeps_by_guard_id(records) do
    Enum.reduce(records, %{guards: %{}, asleep_at: nil}, fn record, state ->
      case record do
        %Record{activity: :falls_asleep} ->
          Map.replace!(state, :asleep_at, record.timestamp)

        %Record{activity: :wakes_up} ->
          update_in(Map.replace!(state, :asleep_at, nil), [:guards, record.guard_id], fn sleeps ->
            new_sleep = state.asleep_at.minute..(record.timestamp.minute - 1)

            if sleeps do
              [new_sleep | sleeps]
            else
              [new_sleep]
            end
          end)

        %Record{activity: :shift_change} ->
          state
      end
    end)
    |> Map.fetch!(:guards)
  end

  defp parse_records(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse/1)
    |> Enum.sort_by(fn record ->
      record |> Map.fetch!(:timestamp) |> DateTime.to_unix()
    end)
    |> Enum.reduce([], fn %Record{guard_id: guard_id} = record, records ->
      updated_record =
        if guard_id do
          record
        else
          Map.replace!(record, :guard_id, hd(records).guard_id)
        end

      [updated_record | records]
    end)
    |> Enum.reverse()
  end

  defp parse(string) do
    [timestamp_string, activity_string | []] =
      String.split(string, ["[", "] "], trim: true, parts: 2)

    {:ok, timestamp, _} = DateTime.from_iso8601(timestamp_string <> ":00Z")

    {activity, guard_id} =
      case activity_string do
        "falls asleep" ->
          {:falls_asleep, nil}

        "wakes up" ->
          {:wakes_up, nil}

        _ ->
          guard_id =
            activity_string
            |> String.split(["#", " "], trim: true)
            |> tl()
            |> hd()
            |> String.to_integer()

          {:shift_change, guard_id}
      end

    %Record{timestamp: timestamp, activity: activity, guard_id: guard_id}
  end
end
