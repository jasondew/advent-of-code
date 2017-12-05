module Tests exposing (..)

import Test exposing (..)
import DayOne
import DayTwo
import DayThree
import DayFour
import Expect
import String


all : Test
all =
    describe "Test Suite"
        [ describe "Day 1" dayOneTests
        , describe "Day 2" dayTwoTests
        , describe "Day 3" dayThreeTests
        , describe "Day 4" dayFourTests
        ]


dayOneTests : List Test
dayOneTests =
    [ test "1122 produces 3" <|
        \() ->
            Expect.equal 3 (DayOne.partOne "1122")
    , test "1111 produces 4" <|
        \() ->
            Expect.equal 4 (DayOne.partOne "1111")
    , test "1234 produces 0" <|
        \() ->
            Expect.equal 0 (DayOne.partOne "1234")
    , test "91212129 produces 9" <|
        \() ->
            Expect.equal 9 (DayOne.partOne "91212129")
    , test "1212 produces 6 with a half-length offset" <|
        \() ->
            Expect.equal 6 (DayOne.partTwo "1212")
    , test "1221 produces 0 with a half-length offset" <|
        \() ->
            Expect.equal 0 (DayOne.partTwo "1221")
    , test "123425 produces 4 with a half-length offset" <|
        \() ->
            Expect.equal 4 (DayOne.partTwo "123425")
    , test "123123 produces 12 with a half-length offset" <|
        \() ->
            Expect.equal 12 (DayOne.partTwo "123123")
    , test "12131415 produces 4 with a half-length offset" <|
        \() ->
            Expect.equal 4 (DayOne.partTwo "12131415")
    ]


dayTwoTests : List Test
dayTwoTests =
    [ test "5 1 9 5; 7 5 3; 2 4 6 8 checksums to 18" <|
        \() -> Expect.equal 18 (DayTwo.partOne "5 1 9 5\n7 5 3\n2 4 6 8")
    , test "5 9 2 8; 9 4 7 3; 3 8 6 5 checksums to 9" <|
        \() -> Expect.equal 9 (DayTwo.partTwo "5 9 2 8\n9 4 7 3\n3 8 6 5")
    ]


dayThreeTests : List Test
dayThreeTests =
    [ test "Data from square 1 is carried 0 steps" <|
        \() -> Expect.equal 0 (DayThree.partOne "1")
    , test "Data from square 12 is carried 3 steps" <|
        \() -> Expect.equal 3 (DayThree.partOne "12")
    , test "Data from square 23 is carried only 2 steps" <|
        \() -> Expect.equal 2 (DayThree.partOne "23")
    , test "Data from square 1024 must be carried 31 steps" <|
        \() -> Expect.equal 31 (DayThree.partOne "1024")
    , test "The first stress test value greater than 10 is 11" <|
        \() -> Expect.equal 11 (DayThree.partTwo "10")
    , test "The first stress test value greater than 25 is 26" <|
        \() -> Expect.equal 26 (DayThree.partTwo "25")
    , test "The first stress test value greater than 26 is 54" <|
        \() -> Expect.equal 54 (DayThree.partTwo "26")
    , test "The first stress test value greater than 130 is 133" <|
        \() -> Expect.equal 133 (DayThree.partTwo "130")
    , test "The first stress test value greater than 150 is 304" <|
        \() -> Expect.equal 304 (DayThree.partTwo "150")
    , test "The first stress test value greater than 800 is 806" <|
        \() -> Expect.equal 806 (DayThree.partTwo "800")
    , test "The first stress test value greater than 44000 is 45220" <|
        \() -> Expect.equal 45220 (DayThree.partTwo "44000")
    ]


dayFourTests : List Test
dayFourTests =
    [ test "aa bb cc dd ee is valid" <|
        \() -> Expect.equal 1 (DayFour.partOne "aa bb cc dd ee\n")
    , test "aa bb cc dd aa is not valid" <|
        \() -> Expect.equal 0 (DayFour.partOne "aa bb cc dd aa")
    , test "aa bb cc dd aaa is valid" <|
        \() -> Expect.equal 1 (DayFour.partOne "aa bb cc dd aaa")
    ]
