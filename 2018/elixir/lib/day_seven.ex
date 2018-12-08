defmodule ElixirSolutions.DaySeven do
  @type step :: String.t()
  @type requirement :: {step, step}

  @doc ~S"""
    Part 1

    iex> ElixirSolutions.DaySeven.part_one("Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.")
    "CABDFE"
  """
  def part_one(input) do
    requirements = parse_requirements(input)
    preconditions = find_preconditions(requirements)

    requirements
    |> find_all_steps
    |> traverse(preconditions)
    |> Enum.reverse()
    |> List.to_string()
  end

  @spec traverse([step], %{required(step) => [step, ...]}) :: [step, ...]
  defp traverse(steps_available, preconditions) do
    traverse(steps_available, preconditions, [])
  end

  @spec traverse([step], %{required(step) => [step, ...]}, [step]) :: [step, ...]
  defp traverse([], _preconditions, steps_taken) do
    steps_taken
  end

  defp traverse(steps, preconditions, steps_taken) do
    next_step =
      steps
      |> possible_next_steps(preconditions)
      |> List.first()

    traverse(
      Enum.reject(steps, &(&1 == next_step)),
      preconditions,
      [next_step | steps_taken]
    )
  end

  # -----------------------------------------------------------------------------

  @doc ~S"""
    Part 2

    iex> ElixirSolutions.DaySeven.part_two("Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.", 2, 0)
    15
  """

  defmodule Worker do
    defstruct [:name, :current_step, :time_left]
  end

  defmodule StepTracker do
    defstruct [:base_time, :pending, :in_progress, :complete]
  end

  def part_two(input, worker_count \\ 5, base_time \\ 60) do
    requirements = parse_requirements(input)
    preconditions = find_preconditions(requirements)

    step_tracker = %StepTracker{
      base_time: base_time,
      pending: find_all_steps(requirements),
      in_progress: [],
      complete: []
    }

    workers =
      for i <- 1..worker_count,
          do: %Worker{name: "Worker #{i}", current_step: nil, time_left: 0}

    print_debug_header_line(worker_count)
    run(preconditions, step_tracker, workers, 0)
  end

  defp run(preconditions, step_tracker, workers, time) do
    if Enum.count(step_tracker.pending) == 0 and Enum.count(step_tracker.in_progress) == 0 do
      IO.puts("DONE!")
      time - 1
    else
      {updated_workers, updated_step_tracker} =
        Enum.map_reduce(workers, step_tracker, fn worker, step_tracker ->
          run_worker(worker, step_tracker, preconditions)
        end)

      print_debug_line(time + 1, updated_workers, updated_step_tracker)

      run(preconditions, updated_step_tracker, updated_workers, time + 1)
    end
  end

  defp run_worker(%Worker{time_left: 0} = worker, step_tracker, preconditions) do
    updated_step_tracker =
      if is_nil(worker.current_step) do
        step_tracker
      else
        %{
          step_tracker
          | in_progress: Enum.reject(step_tracker.in_progress, &(&1 == worker.current_step)),
            complete: [worker.current_step | step_tracker.complete]
        }
      end

    updated_worker = %{worker | current_step: nil, time_left: 0}

    next_step =
      updated_step_tracker.pending
      |> Enum.concat(updated_step_tracker.in_progress)
      |> possible_next_steps(preconditions)
      |> Enum.reject(&Enum.member?(updated_step_tracker.in_progress, &1))
      |> List.first()

    if is_nil(next_step) do
      {updated_worker, updated_step_tracker}
    else
      {
        %{
          updated_worker
          | current_step: next_step,
            time_left: total_time(step_tracker.base_time, next_step) - 1
        },
        %{
          updated_step_tracker
          | pending: Enum.reject(updated_step_tracker.pending, &(&1 == next_step)),
            in_progress: [next_step | updated_step_tracker.in_progress]
        }
      }
    end
  end

  defp run_worker(%Worker{time_left: time_left} = worker, step_tracker, _preconditions) do
    updated_worker = %{worker | time_left: time_left - 1}

    {updated_worker, step_tracker}
  end

  defp total_time(base_time, step_name) do
    step_name
    |> String.to_charlist()
    |> List.first()
    |> (fn code -> code - 64 + base_time end).()
  end

  defp print_debug_header_line(worker_count) do
    ["\n\n  Second   "]
    |> Enum.concat(
      1..worker_count
      |> Enum.map(fn index -> "Worker #{Integer.to_string(index)}" end)
      |> Enum.intersperse("   ")
    )
    |> Enum.concat(["   Done"])
    |> Enum.join()
    |> IO.puts()
  end

  defp print_debug_line(time, workers, step_tracker) do
    ["  #{time |> Integer.to_string() |> String.pad_leading(4, " ")}        "]
    |> Enum.concat(
      workers
      |> Enum.map(fn worker -> String.pad_leading(worker.current_step || ".", 1, " ") end)
      |> Enum.intersperse("          ")
    )
    |> Enum.concat(["       ", step_tracker.complete])
    |> Enum.join()
    |> IO.puts()
  end

  # -----------------------------------------------------------------------------

  @spec possible_next_steps([step, ...], %{required(step) => [step, ...]}) :: [step]
  defp possible_next_steps(steps, preconditions) do
    Enum.reject(steps, fn step ->
      precondition_steps = Map.get(preconditions, step)

      if is_nil(precondition_steps) do
        false
      else
        steps
        |> Enum.reject(&(&1 == step))
        |> Enum.any?(fn other_step ->
          Enum.member?(Map.get(preconditions, step) || [], other_step)
        end)
      end
    end)
  end

  @spec find_preconditions([requirement]) :: %{required(step) => [step, ...]}
  defp find_preconditions(requirements) do
    Enum.reduce(requirements, %{}, fn {step1, step2}, graph ->
      Map.update(graph, step2, [step1], &[step1 | &1])
    end)
  end

  @spec find_all_steps([requirement]) :: [step, ...]
  defp find_all_steps(requirements) do
    requirements
    |> Enum.flat_map(fn {step1, step2} -> [step1, step2] end)
    |> Enum.sort()
    |> Enum.uniq()
  end

  @spec parse_requirements(String.t()) :: [requirement]
  defp parse_requirements(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn line ->
      [_, step1, _, _, _, _, _, step2, _, _] = String.split(line, " ")
      {step1, step2}
    end)
  end
end
