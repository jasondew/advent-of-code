defmodule IntcodeTest do
  use ExUnit.Case

  test "opcode 9" do
    quine =
      Conversions.to_integers(
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
        ","
      )

    assert %{outputs: quine} = Intcode.run(quine, [])
  end

  test "foo" do
    assert %{outputs: [1_219_070_632_396_864]} = run("1102,34915192,34915192,7,4,7,99,0")
  end

  test "outputs large numbers" do
    assert %{outputs: [1_125_899_906_842_624]} = run("104,1125899906842624,99")
  end

  defp run(string) do
    string
    |> Conversions.to_integers(",")
    |> Intcode.run([])
  end
end
