module Tests exposing (..)

import Test exposing (..)
import One
import Expect
import String


all : Test
all =
    describe "Day 1" dayOneTests


dayOneTests : List Test
dayOneTests =
    [ test "1122 produces 3" <|
        \() ->
            Expect.equal 3 (One.repeatingSum One.Next "1122")
    , test "1111 produces 4" <|
        \() ->
            Expect.equal 4 (One.repeatingSum One.Next "1111")
    , test "1234 produces 0" <|
        \() ->
            Expect.equal 0 (One.repeatingSum One.Next "1234")
    , test "91212129 produces 9" <|
        \() ->
            Expect.equal 9 (One.repeatingSum One.Next "91212129")
    , test "1212 produces 6 with a half-length offset" <|
        \() ->
            Expect.equal 6 (One.repeatingSum One.Half "1212")
    , test "1221 produces 0 with a half-length offset" <|
        \() ->
            Expect.equal 0 (One.repeatingSum One.Half "1221")
    , test "123425 produces 4 with a half-length offset" <|
        \() ->
            Expect.equal 4 (One.repeatingSum One.Half "123425")
    , test "123123 produces 12 with a half-length offset" <|
        \() ->
            Expect.equal 12 (One.repeatingSum One.Half "123123")
    , test "12131415 produces 4 with a half-length offset" <|
        \() ->
            Expect.equal 4 (One.repeatingSum One.Half "12131415")
    ]
