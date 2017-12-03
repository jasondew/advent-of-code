module DayThree exposing (partOne)


partOne : Int -> Int
partOne location =
    let
        _ =
            Debug.log "location" location

        n =
            floor ((sqrt (location - 1 |> toFloat) + 1) / 2)
                |> Debug.log "n"

        starting =
            (4 * n * n - 4 * n + 2)
                |> Debug.log "Starting"

        period =
            (2 * n) |> Debug.log "period"

        index =
            ((location - starting + 1) % period)
                |> Debug.log "i"
    in
        if index == period // 2 then
            n
        else if index < period // 2 then
            2 * n - index
        else
            n - 1 + index
