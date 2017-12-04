module DayThree exposing (partOne)


partOne : String -> Int
partOne input =
    case String.toInt input of
        Ok location ->
            spiralDistance location

        Err error ->
            -1


spiralDistance : Int -> Int
spiralDistance location =
    if location <= 1 then
        0
    else
        let
            n =
                floor ((sqrt (location - 1 |> toFloat) + 1) / 2)

            starting =
                (4 * n * n - 4 * n + 2)

            period =
                (2 * n)

            index =
                ((location - starting + 1) % period)
        in
            if index == period // 2 then
                n
            else if index < period // 2 then
                2 * n - index
            else
                2 * n - period + index
