module DayTwo exposing (partOne, partTwo)


partOne : String -> Int
partOne input =
    input
        |> toSpreadsheet
        |> List.map maximumDifference
        |> List.sum


partTwo : String -> Int
partTwo input =
    input
        |> toSpreadsheet
        |> List.map evenlyDivisibleDifference
        |> List.sum


evenlyDivisibleDifference : List Int -> Int
evenlyDivisibleDifference numbers =
    let
        test : Int -> List Int -> Maybe ( Int, Int )
        test candidate rest =
            case rest of
                [] ->
                    Nothing

                next :: untested ->
                    if evenlyDivisible candidate next then
                        Just ( candidate, next )
                    else
                        test candidate untested
    in
        case numbers of
            [] ->
                0

            candidate :: rest ->
                case test candidate rest of
                    Just ( a, b ) ->
                        if a > b then
                            a // b
                        else
                            b // a

                    Nothing ->
                        evenlyDivisibleDifference rest


evenlyDivisible : Int -> Int -> Bool
evenlyDivisible a b =
    if a > b then
        rem a b == 0
    else
        rem b a == 0


maximumDifference : List Int -> Int
maximumDifference numbers =
    let
        maybeMin =
            List.minimum numbers

        maybeMax =
            List.maximum numbers
    in
        Maybe.map2 (\min max -> max - min) maybeMin maybeMax
            |> Maybe.withDefault 0


toSpreadsheet : String -> List (List Int)
toSpreadsheet input =
    input
        |> String.lines
        |> List.map (String.words >> List.map (String.toInt >> Result.withDefault 0))
