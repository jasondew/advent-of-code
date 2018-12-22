defmodule ElixirSolutions.SantaWatch do
  use Bitwise

  def execute(registers, {op_name, a, b, c}) do
    op = get_op(op_name)
    op.(registers, a, b, c)
  end

  defp get_op("addr"), do: &addr/4
  defp get_op("addi"), do: &addi/4
  defp get_op("mulr"), do: &mulr/4
  defp get_op("muli"), do: &muli/4
  defp get_op("banr"), do: &banr/4
  defp get_op("bani"), do: &bani/4
  defp get_op("borr"), do: &borr/4
  defp get_op("bori"), do: &bori/4
  defp get_op("setr"), do: &setr/4
  defp get_op("seti"), do: &seti/4
  defp get_op("gtir"), do: &gtir/4
  defp get_op("gtri"), do: &gtri/4
  defp get_op("gtrr"), do: &gtrr/4
  defp get_op("eqir"), do: &eqir/4
  defp get_op("eqri"), do: &eqri/4
  defp get_op("eqrr"), do: &eqrr/4

  def opcodes_by_name() do
    [
      addr: &addr/4,
      addi: &addi/4,
      mulr: &mulr/4,
      muli: &muli/4,
      banr: &banr/4,
      bani: &bani/4,
      borr: &borr/4,
      bori: &bori/4,
      setr: &setr/4,
      seti: &seti/4,
      gtir: &gtir/4,
      gtri: &gtri/4,
      gtrr: &gtrr/4,
      eqir: &eqir/4,
      eqri: &eqri/4,
      eqrr: &eqrr/4
    ]
  end

  def all_opcodes(), do: for({_name, fun} <- opcodes_by_name(), do: fun)

  def get(registers, id) when is_integer(id) do
    Enum.at(registers, id)
  end

  def set(registers, id, value) when is_integer(id) do
    List.replace_at(registers, id, value)
  end

  @doc ~S"""
    addr (add register) stores into register C the result of adding register A and register B.
    iex> addr([0, 1, 2, 3], 0, 1, 3)
    [0, 1, 2, 1]
  """
  def addr(registers, a, b, c) do
    set(registers, c, get(registers, a) + get(registers, b))
  end

  @doc ~S"""
    addi (add immediate) stores into register C the result of adding register A and value B.
    iex> addi([0, 1, 2, 3], 1, 7, 3)
    [0, 1, 2, 8]
  """
  def addi(registers, a, b, c) do
    set(registers, c, get(registers, a) + b)
  end

  @doc ~S"""
    mulr (multiply register) stores into register C the result of multiplying register A and register B.
    iex> mulr([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 2]
  """
  def mulr(registers, a, b, c) do
    set(registers, c, get(registers, a) * get(registers, b))
  end

  @doc ~S"""
    muli (multiply immediate) stores into register C the result of multiplying register A and value B.
    iex> muli([0, 1, 2, 3], 1, 7, 3)
    [0, 1, 2, 7]
  """
  def muli(registers, a, b, c) do
    set(registers, c, get(registers, a) * b)
  end

  @doc ~S"""
    banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    iex> banr([0, 1, 2, 3], 0, 2, 3)
    [0, 1, 2, 0]
  """
  def banr(registers, a, b, c) do
    set(registers, c, get(registers, a) &&& get(registers, b))
  end

  @doc ~S"""
    bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    iex> bani([0, 1, 2, 3], 2, 3, 3)
    [0, 1, 2, 2]
  """
  def bani(registers, a, b, c) do
    set(registers, c, get(registers, a) &&& b)
  end

  @doc ~S"""
    borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    iex> borr([0, 1, 2, 3], 0, 1, 3)
    [0, 1, 2, 1]
  """
  def borr(registers, a, b, c) do
    set(registers, c, get(registers, a) ||| get(registers, b))
  end

  @doc ~S"""
    bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    iex> bori([0, 1, 2, 3], 3, 9, 3)
    [0, 1, 2, 11]
  """
  def bori(registers, a, b, c) do
    set(registers, c, get(registers, a) ||| b)
  end

  @doc ~S"""
    setr (set register) copies the contents of register A into register C. (Input B is ignored.)
    iex> setr([0, 1, 2, 3], 1, 42, 3)
    [0, 1, 2, 1]
  """
  def setr(registers, a, _b, c) do
    set(registers, c, get(registers, a))
  end

  @doc ~S"""
    seti (set immediate) stores value A into register C. (Input B is ignored.)
    iex> seti([0, 1, 2, 3], 8, 42, 0)
    [8, 1, 2, 3]
  """
  def seti(registers, a, _b, c) do
    set(registers, c, a)
  end

  @doc ~S"""
    gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    iex> gtir([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 0]

    iex> gtir([0, 1, 2, 3], 1, 0, 3)
    [0, 1, 2, 1]
  """
  def gtir(registers, a, b, c) do
    set(registers, c, if(a > get(registers, b), do: 1, else: 0))
  end

  @doc ~S"""
    gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    iex> gtri([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 0]

    iex> gtri([0, 1, 2, 3], 2, 1, 3)
    [0, 1, 2, 1]
  """
  def gtri(registers, a, b, c) do
    set(registers, c, if(get(registers, a) > b, do: 1, else: 0))
  end

  @doc ~S"""
    gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    iex> gtrr([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 0]

    iex> gtrr([0, 1, 2, 3], 2, 1, 3)
    [0, 1, 2, 1]
  """
  def gtrr(registers, a, b, c) do
    set(
      registers,
      c,
      if(get(registers, a) > get(registers, b), do: 1, else: 0)
    )
  end

  @doc ~S"""
    eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    iex> eqir([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 1]

    iex> eqir([0, 1, 2, 3], 0, 1, 3)
    [0, 1, 2, 0]
  """
  def eqir(registers, a, b, c) do
    set(registers, c, if(a == get(registers, b), do: 1, else: 0))
  end

  @doc ~S"""
    eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    iex> eqri([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 1]

    iex> eqri([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 0]
  """
  def eqri(registers, a, b, c) do
    set(registers, c, if(get(registers, a) == b, do: 1, else: 0))
  end

  @doc ~S"""
    eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    iex> eqrr([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 1]

    iex> eqrr([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 0]
  """
  def eqrr(registers, a, b, c) do
    set(
      registers,
      c,
      if(get(registers, a) == get(registers, b), do: 1, else: 0)
    )
  end
end
