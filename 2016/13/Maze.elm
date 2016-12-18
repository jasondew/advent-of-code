module Maze exposing (main)

import Html exposing (..)
import Html.Events exposing (onInput)
import Html.Attributes exposing (style)
import List.Extra
import Bitwise


type alias Model =
    { favoriteNumber : Int
    , maze : Maze
    , path : Maybe Path
    , accessibleCount : Int
    }


type alias Maze =
    List (List Cell)


type alias Path =
    List Position


type Cell
    = Open
    | Blocked


type alias Position =
    ( Int, Int )


type Msg
    = NewInput String


initialInput : String
initialInput =
    "1358"


init : ( Model, Cmd Msg )
init =
    update (NewInput initialInput) (Model 0 [] Nothing 0)


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewInput input ->
            case String.toInt input of
                Ok value ->
                    let
                        maze =
                            generateMaze value
                    in
                        { model
                            | favoriteNumber = value
                            , maze = maze
                            , path = shortestPath maze 100 0 ( 31, 39 ) [] initialPosition
                            , accessibleCount = accessibleCount maze
                        }
                            ! []

                Err _ ->
                    model ! []


mazeSize : Int
mazeSize =
    50


initialPosition : Position
initialPosition =
    ( 1, 1 )


generateMaze : Int -> Maze
generateMaze seed =
    List.map
        (\columnIndex ->
            List.map
                (\rowIndex ->
                    generateCell seed rowIndex columnIndex
                )
                (List.range 0 (mazeSize - 1))
        )
        (List.range 0 (mazeSize - 1))


generateCell : Int -> Int -> Int -> Cell
generateCell seed x y =
    if bitSetCount (polynomial seed x y) % 2 == 0 then
        Open
    else
        Blocked


polynomial : Int -> Int -> Int -> Int
polynomial seed x y =
    x * x + 3 * x + 2 * x * y + y + y * y + seed


bitSetCount : Int -> Int
bitSetCount value =
    let
        powersOfTwo =
            List.map (\index -> Bitwise.shiftLeftBy index 1) (List.range 0 13)

        masks =
            List.map2 Bitwise.and powersOfTwo (List.repeat (List.length powersOfTwo) value)

        bits =
            List.filter (\x -> x > 0) masks
    in
        List.length bits


shortestPath : Maze -> Int -> Int -> Position -> Path -> Position -> Maybe Path
shortestPath maze maxIterations iteration ( gx, gy ) path ( x, y ) =
    if iteration >= maxIterations then
        Debug.log "Too many iterations, stopping." Nothing
    else if (x < 0 || x >= mazeSize) || (y < 0 || y >= mazeSize) then
        Nothing
    else if List.member ( x, y ) path then
        Nothing
    else if x == gx && y == gy then
        Just <| ( gx, gy ) :: path
    else if (cellBlockedAt maze ( x, y )) then
        Nothing
    else
        case shortestPath maze maxIterations (iteration + 1) ( gx, gy ) (( x, y ) :: path) ( x, y + 1 ) of
            Just path ->
                Just path

            _ ->
                case shortestPath maze maxIterations (iteration + 1) ( gx, gy ) (( x, y ) :: path) ( x - 1, y ) of
                    Just path ->
                        Just path

                    _ ->
                        case shortestPath maze maxIterations (iteration + 1) ( gx, gy ) (( x, y ) :: path) ( x, y - 1 ) of
                            Just path ->
                                Just path

                            _ ->
                                case shortestPath maze maxIterations (iteration + 1) ( gx, gy ) (( x, y ) :: path) ( x + 1, y ) of
                                    Just path ->
                                        Just path

                                    _ ->
                                        Nothing


cellBlockedAt : Maze -> Position -> Bool
cellBlockedAt maze ( x, y ) =
    case List.Extra.getAt y maze of
        Just column ->
            case List.Extra.getAt x column of
                Just cell ->
                    cell == Blocked

                Nothing ->
                    True

        Nothing ->
            True


accessibleCount : Maze -> Int
accessibleCount maze =
    let
        allPositions =
            List.foldl
                (\row positions ->
                    List.foldl
                        (\column positions ->
                            ( row, column ) :: positions
                        )
                        positions
                        (List.range 0 (mazeSize - 1) |> List.reverse)
                )
                []
                (List.range 0 (mazeSize - 1) |> List.reverse)
                |> List.filter (\position -> not <| cellBlockedAt maze position)
                |> List.filter (\( x, y ) -> not <| (cellBlockedAt maze ( x + 1, y )) && (cellBlockedAt maze ( x - 1, y )) && (cellBlockedAt maze ( x, y + 1 )) && (cellBlockedAt maze ( x, y - 1 )))
    in
        List.foldl
            (\position accessiblePositions ->
                case shortestPath maze 75 0 position [] initialPosition of
                    Just path ->
                        List.append (List.take 50 path) accessiblePositions |> List.Extra.unique

                    Nothing ->
                        accessiblePositions
            )
            []
            allPositions
            |> List.length


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 13" ]
        , div
            []
            [ div
                []
                [ label [] [ text "Favorite number: " ]
                , textarea [ onInput NewInput ] [ text <| toString model.favoriteNumber ]
                ]
            ]
        , h3 [] [ text "Steps: ", ((model.path |> Maybe.withDefault [] |> List.length) - 1) |> toString |> text ]
        , h3 [] [ text "Accessible: ", model.accessibleCount |> toString |> text ]
        , mazeView model.maze (model.path |> Maybe.withDefault [])
        ]


mazeView : Maze -> Path -> Html Msg
mazeView maze path =
    List.indexedMap
        (\columnIndex column ->
            List.indexedMap
                (\rowIndex cell -> cellView (List.member ( rowIndex, columnIndex ) path) cell)
                column
                |> tr []
        )
        maze
        |> table [ style [ ( "border-collapse", "collapse" ), ( "border", "1px solid black" ) ] ]


cellView : Bool -> Cell -> Html Msg
cellView pathCell cell =
    let
        color =
            if pathCell then
                "red"
            else
                case cell of
                    Open ->
                        "white"

                    Blocked ->
                        "black"
    in
        td
            [ style
                [ ( "background", color )
                , ( "padding", "0" )
                , ( "width", "1em" )
                , ( "height", "1em" )
                ]
            ]
            [ text "\x2003" ]


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
