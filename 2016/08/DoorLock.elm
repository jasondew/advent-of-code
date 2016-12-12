module DoorLock exposing (main)

import Html exposing (..)
import Html.Events exposing (onInput)
import Html.Attributes exposing (style)
import List.Extra


type alias Model =
    { instructions : List Instruction
    , grid : Grid
    , onCount : Int
    }


type Instruction
    = Rect Int Int
    | RotateRow Int Int
    | RotateColumn Int Int
    | Error String


type alias Grid =
    List (List Cell)


type Cell
    = On
    | Off


type Msg
    = NewInput String


initialInput : String
initialInput =
    "rect 1x1\nrotate row y=0 by 5\nrect 1x1\nrotate row y=0 by 5\nrect 1x1\nrotate row y=0 by 5\nrect 1x1\nrotate row y=0 by 5\nrect 1x1\nrotate row y=0 by 2\nrect 1x1\nrotate row y=0 by 2\nrect 1x1\nrotate row y=0 by 3\nrect 1x1\nrotate row y=0 by 3\nrect 2x1\nrotate row y=0 by 2\nrect 1x1\nrotate row y=0 by 3\nrect 2x1\nrotate row y=0 by 2\nrect 1x1\nrotate row y=0 by 3\nrect 2x1\nrotate row y=0 by 5\nrect 4x1\nrotate row y=0 by 5\nrotate column x=0 by 1"


init : ( Model, Cmd Msg )
init =
    update (NewInput initialInput) (Model [] initialGrid 0)


initialGrid : Grid
initialGrid =
    List.repeat 6 (List.repeat 50 Off)


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewInput input ->
            let
                newInstructions =
                    decode input

                newGrid =
                    execute initialGrid newInstructions
            in
                { model
                    | instructions = newInstructions
                    , grid = newGrid
                    , onCount = onCount newGrid
                }
                    ! []


onCount : Grid -> Int
onCount grid =
    List.foldl
        (\row count ->
            count
                + List.foldl
                    (\cell count ->
                        case cell of
                            On ->
                                count + 1

                            Off ->
                                count
                    )
                    0
                    row
        )
        0
        grid


decode : String -> List Instruction
decode input =
    input
        |> String.lines
        |> List.map decodeInstruction


decodeInstruction : String -> Instruction
decodeInstruction string =
    if String.startsWith "rect" string then
        decodeRect <| String.dropLeft 5 string
    else if String.startsWith "rotate row" string then
        decodeRotateRow <| String.dropLeft 13 string
    else if String.startsWith "rotate column" string then
        decodeRotateColumn <| String.dropLeft 16 string
    else
        Error string


decodeRect : String -> Instruction
decodeRect string =
    let
        values =
            string
                |> String.split "x"
                |> List.map (String.toInt >> Result.withDefault 0)
    in
        case values of
            columns :: rows :: [] ->
                Rect columns rows

            _ ->
                Error string


decodeRotateRow : String -> Instruction
decodeRotateRow string =
    let
        values =
            string
                |> String.split " "
                |> List.map (String.toInt >> Result.withDefault 0)
    in
        case values of
            location :: _ :: by :: [] ->
                RotateRow location by

            _ ->
                Error string


decodeRotateColumn : String -> Instruction
decodeRotateColumn string =
    let
        values =
            string
                |> String.split " "
                |> List.map (String.toInt >> Result.withDefault 0)
    in
        case values of
            location :: _ :: by :: [] ->
                RotateColumn location by

            _ ->
                Error string


rect : Int -> Int -> Grid -> Grid
rect columns rows grid =
    grid
        |> List.indexedMap
            (\index row ->
                if index < rows then
                    turnOnColumns columns row
                else
                    row
            )


turnOnColumns : Int -> List Cell -> List Cell
turnOnColumns columns row =
    List.indexedMap
        (\index cell ->
            if index < columns then
                On
            else
                cell
        )
        row


rotateRow : Int -> Int -> Grid -> Grid
rotateRow location amount grid =
    grid
        |> List.indexedMap
            (\index row ->
                if index == location then
                    rotateColumns row amount
                else
                    row
            )


rotateColumns : List Cell -> Int -> List Cell
rotateColumns row amount =
    List.append (List.reverse row |> List.take amount |> List.reverse)
        (List.take (List.length row - amount) row)


rotateColumn : Int -> Int -> Grid -> Grid
rotateColumn location amount grid =
    grid
        |> List.Extra.transpose
        |> rotateRow location amount
        |> List.Extra.transpose


execute : Grid -> List Instruction -> Grid
execute grid instructions =
    List.foldl executeInstruction grid instructions


executeInstruction : Instruction -> Grid -> Grid
executeInstruction instruction grid =
    case instruction of
        Rect columns rows ->
            rect columns rows grid

        RotateRow location amount ->
            rotateRow location amount grid

        RotateColumn location amount ->
            rotateColumn location amount grid

        Error _ ->
            grid


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 8" ]
        , div
            []
            [ div
                []
                [ label [] [ text "Input: " ]
                , textarea [ onInput NewInput ] [ text initialInput ]
                ]
            ]
        , h3 [] [ text "On Count: ", text <| toString model.onCount ]
        , gridView model.grid
        , instructionsView model.instructions
        ]


gridView : Grid -> Html Msg
gridView grid =
    grid
        |> List.map rowView
        |> table []


rowView : List Cell -> Html Msg
rowView row =
    row
        |> List.map cellView
        |> tr []


cellView : Cell -> Html Msg
cellView cell =
    case cell of
        On ->
            td [ style [ ( "background", "red" ) ] ] [ text "" ]

        Off ->
            td [ style [ ( "background", "black" ) ] ] [ text "" ]


instructionsView : List Instruction -> Html Msg
instructionsView instructions =
    instructions
        |> List.map instructionView
        |> table []


instructionView : Instruction -> Html Msg
instructionView instruction =
    (case instruction of
        Rect columns rows ->
            [ td [] [ text "Rect" ]
            , td [] [ text <| (toString columns) ++ " by " ++ (toString rows) ]
            ]

        RotateColumn location amount ->
            [ td [] [ text "RotateColumn" ]
            , td [] [ text <| "at x=" ++ (toString location) ++ " by " ++ (toString amount) ]
            ]

        RotateRow location amount ->
            [ td [] [ text "RotateRow" ]
            , td [] [ text <| "at y=" ++ (toString location) ++ " by " ++ (toString amount) ]
            ]

        Error input ->
            [ td [] [ text "ERROR" ]
            , td [] [ text input ]
            ]
    )
        |> tr []


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
