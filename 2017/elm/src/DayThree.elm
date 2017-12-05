module DayThree exposing (partOne, partTwo)

import List.Extra


partOne : String -> Int
partOne input =
    case String.toInt input of
        Ok location ->
            spiralDistance location

        Err _ ->
            -1


partTwo : String -> Int
partTwo input =
    case String.toInt input of
        Ok target ->
            firstSprialSumGreaterThan target initialRingSums

        Err _ ->
            -1


spiralDistance : Int -> Int
spiralDistance location =
    if location <= 1 then
        0
    else
        let
            ring =
                ringNumber location

            period =
                (2 * ring)

            index =
                ((location - ringStartingValue ring) % period) + 1
        in
            if index == period // 2 then
                ring
            else if index < period // 2 then
                2 * ring - index
            else
                2 * ring - (period - index)


ringNumber : Int -> Int
ringNumber location =
    floor ((sqrt (location - 1 |> toFloat) + 1) / 2)


ringIndex : Int -> Int -> Int
ringIndex ring location =
    let
        period =
            8 * ring
    in
        (location - ringStartingValue ring) % period


ringStartingValue : Int -> Int
ringStartingValue ring =
    (4 * ring * ring - 4 * ring + 2)


ringLastIndex : Int -> Int
ringLastIndex ring =
    ring * 8 - 1


type RingLocation
    = Top
    | TopRight
    | TopRightCorner
    | RightTop
    | Right
    | RightBottom
    | BottomRightCorner
    | BottomRight
    | Bottom
    | BottomLeft
    | BottomLeftCorner
    | LeftBottom
    | Left
    | LeftTop
    | TopLeftCorner
    | TopLeft


initialRingSums : ( Int, List Int )
initialRingSums =
    ( 25, [ 23, 11, 10, 5, 4, 2, 1, 1 ] )


firstSprialSumGreaterThan : Int -> ( Int, List Int ) -> Int
firstSprialSumGreaterThan target ( highestSum, smallerSpiralSums ) =
    if target < 25 then
        List.Extra.find (\x -> x > target) (List.reverse smallerSpiralSums)
            |> Maybe.withDefault 0
    else if highestSum > target then
        highestSum
    else
        let
            previousSums =
                highestSum :: smallerSpiralSums

            nextSum =
                findNextSprialSum previousSums
        in
            firstSprialSumGreaterThan target ( nextSum, previousSums )


findNextSprialSum : List Int -> Int
findNextSprialSum sums =
    let
        location =
            ((List.length sums) + 1)

        ring =
            ringNumber location

        index =
            ringIndex ring location

        starting =
            ringStartingValue ring

        ringLocation =
            let
                between x ( a, b ) =
                    x >= a && x <= b

                sideRunLength =
                    2 * ring - 3
            in
                if index == 0 then
                    RightBottom
                else if between index ( 1, sideRunLength ) then
                    Right
                else if index == sideRunLength + 1 then
                    RightTop
                else if index == sideRunLength + 2 then
                    TopRightCorner
                else if index == sideRunLength + 3 then
                    TopRight
                else if between index ( sideRunLength + 4, sideRunLength * 2 + 3 ) then
                    Top
                else if index == sideRunLength * 2 + 4 then
                    TopLeft
                else if index == sideRunLength * 2 + 5 then
                    TopLeftCorner
                else if index == sideRunLength * 2 + 6 then
                    LeftTop
                else if between index ( sideRunLength * 2 + 7, sideRunLength * 3 + 6 ) then
                    Left
                else if index == sideRunLength * 3 + 7 then
                    LeftBottom
                else if index == sideRunLength * 3 + 8 then
                    BottomLeftCorner
                else if index == sideRunLength * 3 + 9 then
                    BottomLeft
                else if between index ( sideRunLength * 3 + 10, sideRunLength * 4 + 9 ) then
                    Bottom
                else if index == sideRunLength * 4 + 10 then
                    BottomRight
                else
                    BottomRightCorner

        listOrder ring index =
            ringStartingValue ring + index - 1

        spiralSum : Int -> Int -> Int
        spiralSum ring index =
            let
                order =
                    listOrder ring (index % (ringLastIndex ring + 1))
            in
                case List.Extra.getAt order (List.reverse sums) of
                    Just value ->
                        value

                    Nothing ->
                        ("Can't get index " ++ toString order ++ " of " ++ toString sums)
                            |> Debug.crash

        ringsAndIndices =
            case ringLocation of
                Top ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 2 )
                    , ( ring - 1, index - 3 )
                    , ( ring - 1, index - 4 )
                    ]

                TopRight ->
                    [ ( ring, index - 1 )
                    , ( ring, index - 2 )
                    , ( ring - 1, index - 2 )
                    , ( ring - 1, index - 3 )
                    ]

                TopRightCorner ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 2 )
                    ]

                RightTop ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 1 )
                    , ( ring - 1, index - 2 )
                    ]

                Right ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index )
                    , ( ring - 1, index - 1 )
                    , ( ring - 1, index - 2 )
                    ]

                RightBottom ->
                    [ ( ring - 1, 0 )
                    , ( ring - 1, ringLastIndex <| ring - 1 )
                    ]

                BottomRightCorner ->
                    [ ( ring, 0 )
                    , ( ring, index - 1 )
                    , ( ring - 1, ringLastIndex <| ring - 1 )
                    ]

                BottomRight ->
                    [ ( ring, index - 1 )
                    , ( ring, 0 )
                    , ( ring - 1, index - 8 )
                    , ( ring - 1, index - 7 )
                    ]

                Bottom ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 6 )
                    , ( ring - 1, index - 7 )
                    , ( ring - 1, index - 8 )
                    ]

                BottomLeft ->
                    [ ( ring, index - 1 )
                    , ( ring, index - 2 )
                    , ( ring - 1, index - 6 )
                    , ( ring - 1, index - 7 )
                    ]

                BottomLeftCorner ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 6 )
                    ]

                LeftBottom ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 5 )
                    , ( ring - 1, index - 6 )
                    ]

                Left ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 4 )
                    , ( ring - 1, index - 5 )
                    , ( ring - 1, index - 6 )
                    ]

                LeftTop ->
                    [ ( ring, index - 1 )
                    , ( ring, index - 2 )
                    , ( ring - 1, index - 4 )
                    , ( ring - 1, index - 5 )
                    ]

                TopLeftCorner ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 4 )
                    ]

                TopLeft ->
                    [ ( ring, index - 1 )
                    , ( ring - 1, index - 3 )
                    , ( ring - 1, index - 4 )
                    ]
    in
        ringsAndIndices
            --            |> Debug.log ("location: " ++ toString location ++ " -> (" ++ toString ring ++ ", " ++ toString index ++ ")    " ++ toString ringLocation)
            |> List.map (\( ring, index ) -> spiralSum ring index)
            --            |> Debug.log "summands"
            |> List.sum



--            |> Debug.log "nextSum"
