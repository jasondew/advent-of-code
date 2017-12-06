module DayFive exposing (partOne, partTwo)

import List.Extra


type alias JumpsZipper =
    ( List Int, Maybe Int, List Int )


type alias State =
    { counter : Int
    , jumps : JumpsZipper
    }


partOne : String -> Int
partOne input =
    input
        |> toInts
        |> initialState
        |> run (\jump -> jump + 1)
        |> .counter


partTwo : String -> Int
partTwo input =
    input
        |> toInts
        |> initialState
        |> run
            (\jump ->
                if jump > 2 then
                    jump - 1
                else
                    jump + 1
            )
        |> .counter


toInts : String -> List Int
toInts input =
    input
        |> String.words
        |> List.filterMap (String.toInt >> Result.toMaybe)


maximumIterations : Int
maximumIterations =
    35000000


initialState : List Int -> State
initialState jumps =
    let
        zipperListJumps =
            case jumps of
                [] ->
                    ( [], Nothing, [] )

                first :: rest ->
                    ( [], Just first, rest )
    in
        State 0 zipperListJumps


run : (Int -> Int) -> State -> State
run updateRule ({ counter, jumps } as state) =
    if counter >= maximumIterations then
        state
    else
        case jumps of
            ( _, Nothing, _ ) ->
                state

            ( left, Just current, right ) ->
                { state | jumps = step updateRule left current right, counter = counter + 1 }
                    --                    |> Debug.log "new state"
                    |> run updateRule


step : (Int -> Int) -> List Int -> Int -> List Int -> JumpsZipper
step updateRule left current right =
    if current == 0 then
        ( left, Just (updateRule current), right )
    else
        let
            list =
                List.append left ((updateRule current) :: right)

            newPivotIndex =
                (List.length left + current)

            ( newLeft, newRight ) =
                List.Extra.splitAt newPivotIndex list
        in
            case newRight of
                newCurrent :: restRights ->
                    ( newLeft, Just newCurrent, restRights )

                [] ->
                    ( newLeft, Nothing, [] )
