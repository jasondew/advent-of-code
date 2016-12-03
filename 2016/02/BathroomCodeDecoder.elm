module BathroomCodeDecoder exposing (decode, Keypad(..))

-- simple                  advanced
--    | 0  1  2        | 0  1  2  3  4
-- ---+---------    ---+---------------
--  0 | 1  2  3      0 |       1
--  1 | 4  5  6      1 |    2  3  4
--  2 | 7  8  9      2 | 5  6  7  8  9
--                   3 |    A  B  C
--                   4 |       D


type Keypad
    = Simple
    | Advanced


decode : Keypad -> String -> String
decode keypad instructions =
    instructions
        |> String.lines
        |> List.foldl (decodeInstruction keypad) []
        |> List.map (stateToCode keypad)
        |> List.reverse
        |> String.concat


decodeInstruction : Keypad -> String -> List State -> List State
decodeInstruction keypad instruction states =
    let
        previousState =
            List.head states |> Maybe.withDefault (initialState keypad)

        newState =
            String.foldl (decodeStep keypad) previousState instruction
    in
        newState :: states


decodeStep : Keypad -> Char -> State -> State
decodeStep keypad direction state =
    let
        moveFunction =
            case direction of
                'U' ->
                    moveUp

                'D' ->
                    moveDown

                'L' ->
                    moveLeft

                'R' ->
                    moveRight

                _ ->
                    identity
    in
        moveFunction state |> stayIfInvalid keypad state


moveUp : State -> State
moveUp ( x, y ) =
    ( x, y - 1 )


moveDown : State -> State
moveDown ( x, y ) =
    ( x, y + 1 )


moveLeft : State -> State
moveLeft ( x, y ) =
    ( x - 1, y )


moveRight : State -> State
moveRight ( x, y ) =
    ( x + 1, y )


stayIfInvalid : Keypad -> State -> State -> State
stayIfInvalid keypad oldState newState =
    if invalidState keypad newState then
        oldState
    else
        newState


invalidState : Keypad -> State -> Bool
invalidState keypad state =
    not <| List.member state <| validStates keypad


validStates : Keypad -> List State
validStates keypad =
    case keypad of
        Simple ->
            [ ( 0, 0 )
            , ( 0, 1 )
            , ( 0, 2 )
            , ( 1, 0 )
            , ( 1, 1 )
            , ( 1, 2 )
            , ( 2, 0 )
            , ( 2, 1 )
            , ( 2, 2 )
            ]

        Advanced ->
            [ ( 0, 2 )
            , ( 1, 1 )
            , ( 1, 2 )
            , ( 1, 3 )
            , ( 2, 0 )
            , ( 2, 1 )
            , ( 2, 2 )
            , ( 2, 3 )
            , ( 2, 4 )
            , ( 3, 1 )
            , ( 3, 2 )
            , ( 3, 3 )
            , ( 4, 2 )
            ]


type alias State =
    ( Int, Int )


initialState : Keypad -> State
initialState keypad =
    case keypad of
        Simple ->
            ( 1, 1 )

        Advanced ->
            ( 0, 2 )


stateToCode : Keypad -> State -> String
stateToCode keypad ( x, y ) =
    case keypad of
        Simple ->
            toString <|
                case y of
                    0 ->
                        x + 1

                    1 ->
                        x + 4

                    2 ->
                        x + 7

                    _ ->
                        0

        Advanced ->
            case y of
                0 ->
                    "1"

                1 ->
                    x + 1 |> toString

                2 ->
                    x + 5 |> toString

                3 ->
                    case x of
                        1 ->
                            "A"

                        2 ->
                            "B"

                        3 ->
                            "C"

                        _ ->
                            "?"

                4 ->
                    "D"

                _ ->
                    "?"
