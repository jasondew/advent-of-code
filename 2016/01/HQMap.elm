module First exposing (..)

import Html exposing (..)
import Html.Events exposing (onInput)
import List.Extra as ListExtra
import Color
import Collage
import Element


type alias Model =
    { directions : List Direction
    , stateHistory : StateHistory
    }


type Msg
    = NewDirections String


type alias Direction =
    String


type alias Position =
    ( Int, Int )


type alias Path =
    ( Position, Position )


type Heading
    = North
    | East
    | South
    | West


type alias State =
    { step : Int
    , position : Position
    , heading : Heading
    }


type alias StateHistory =
    List ( Direction, State )


init : ( Model, Cmd Msg )
init =
    ( Model [] [], Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewDirections directionsString ->
            let
                newDirections =
                    String.split ", " directionsString

                newStateHistory =
                    List.scanl stepState ( "initial", initialState ) newDirections
            in
                { model | directions = newDirections, stateHistory = newStateHistory } ! []


initialState : State
initialState =
    State 0 ( 0, 0 ) North


stepState : Direction -> ( Direction, State ) -> ( Direction, State )
stepState direction ( _, { step, position, heading } ) =
    let
        turn =
            String.left 1 direction

        blocks =
            String.dropLeft 1 direction |> String.toInt |> Result.withDefault 0

        newHeading =
            stepHeading heading turn

        newPosition =
            stepPosition position newHeading blocks
    in
        ( direction
        , { step = step + 1
          , position = newPosition
          , heading = newHeading
          }
        )


stepPosition : Position -> Heading -> Int -> Position
stepPosition ( x, y ) heading blocks =
    case heading of
        North ->
            ( x, y + blocks )

        East ->
            ( x + blocks, y )

        South ->
            ( x, y - blocks )

        West ->
            ( x - blocks, y )


stepHeading : Heading -> String -> Heading
stepHeading heading turn =
    case turn of
        "L" ->
            rotateLeft heading

        "R" ->
            rotateRight heading

        _ ->
            heading


rotateRight : Heading -> Heading
rotateRight heading =
    case heading of
        North ->
            East

        East ->
            South

        South ->
            West

        West ->
            North


rotateLeft : Heading -> Heading
rotateLeft heading =
    case heading of
        North ->
            West

        East ->
            North

        South ->
            East

        West ->
            South


finalPosition : StateHistory -> Position
finalPosition stateHistory =
    stateHistory
        |> List.reverse
        |> List.head
        |> Maybe.withDefault ( "initial", initialState )
        |> Tuple.second
        |> .position


distanceFromOrigin : Position -> Int
distanceFromOrigin ( x, y ) =
    (abs x) + (abs y)


firstPositionRevisited : StateHistory -> Maybe Position
firstPositionRevisited stateHistory =
    stateHistory
        |> List.foldl findIntersection ( [], Nothing )
        |> Tuple.second


findIntersection : ( Direction, State ) -> ( List Position, Maybe Position ) -> ( List Position, Maybe Position )
findIntersection ( _, { step, position, heading } ) ( visitedPositions, maybePosition ) =
    case maybePosition of
        Just position ->
            ( [], maybePosition )

        Nothing ->
            let
                newVisitedPositions =
                    position :: visitedPositions
            in
                case (findIntersectingPosition <| toPaths newVisitedPositions) of
                    Just intersectingPosition ->
                        ( [], Just intersectingPosition )

                    Nothing ->
                        ( newVisitedPositions, Nothing )


toPaths : List Position -> List Path
toPaths positions =
    List.map2 (,) positions (List.tail positions |> Maybe.withDefault [])


findIntersectingPosition : List Path -> Maybe Position
findIntersectingPosition paths =
    paths
        |> ListExtra.select
        |> List.foldl
            (\( path, otherPaths ) maybePaths ->
                case maybePaths of
                    Just _ ->
                        maybePaths

                    Nothing ->
                        case ListExtra.find (intersecting path) otherPaths of
                            Just intersectingPath ->
                                Just ( path, intersectingPath )

                            Nothing ->
                                Nothing
            )
            Nothing
        |> Maybe.map (\( pathA, pathB ) -> intersection pathA pathB)


intersection : Path -> Path -> Position
intersection ( a, b ) ( x, y ) =
    ListExtra.find (\p -> List.member p (positionsBetween a b)) (positionsBetween x y)
        |> Maybe.withDefault ( 0, 0 )


intersecting : Path -> Path -> Bool
intersecting ( a, b ) ( x, y ) =
    List.any (\p -> List.member p (positionsBetween a b)) (positionsBetween x y)


positionsBetween : Position -> Position -> List Position
positionsBetween ( ax, ay ) ( bx, by ) =
    if ax == bx then
        List.map (\y -> ( ax, y )) (range ay by)
    else
        List.map (\x -> ( x, ay )) (range ax bx)


range : Int -> Int -> List Int
range a b =
    if a <= b then
        List.range (a + 1) (b - 1)
    else
        List.range (b + 1) (a - 1) |> List.reverse


positionView : Position -> String
positionView ( x, y ) =
    "(" ++ (toString x) ++ ", " ++ (toString y) ++ ")"


maybePositionView : Maybe Position -> String
maybePositionView maybePosition =
    case maybePosition of
        Just position ->
            positionView position

        Nothing ->
            "nothing"


stateView : ( Direction, State ) -> Html Msg
stateView ( direction, state ) =
    tr
        []
        [ td [] [ text <| toString state.step ]
        , td [] [ text direction ]
        , td [] [ text <| toString state.heading ]
        , td [] [ text <| positionView state.position ]
        ]


stateHistoryView : StateHistory -> Html Msg
stateHistoryView stateHistory =
    [ thead
        []
        [ th [] [ text "Step" ]
        , th [] [ text "Direction" ]
        , th [] [ text "Heading" ]
        , th [] [ text "Position" ]
        ]
    , tbody [] (List.map stateView stateHistory)
    ]
        |> table []


line : Position -> Position -> Collage.Form
line ( x1, y1 ) ( x2, y2 ) =
    Collage.segment ( toFloat x1, toFloat y1 ) ( toFloat x2, toFloat y2 )
        |> Collage.traced (Collage.solid Color.red)


mapView : StateHistory -> Html Msg
mapView stateHistory =
    List.map2 (\( _, state1 ) ( _, state2 ) -> line state1.position state2.position) stateHistory (List.tail stateHistory |> Maybe.withDefault [])
        |> Collage.collage 500 500
        |> Element.color Color.green
        |> Element.toHtml


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 1, puzzle 1" ]
        , span [] [ label [] [ text "Directions: ", input [ onInput NewDirections ] [] ] ]
        , h3 [] [ text <| "Distance to HQ: " ++ (model.stateHistory |> finalPosition |> distanceFromOrigin |> toString) ]
        , h3 [] [ text <| "First revisit: " ++ (firstPositionRevisited model.stateHistory |> maybePositionView) ]
        , h3 [] [ text <| "Distance to first revisit: " ++ (firstPositionRevisited model.stateHistory |> Maybe.map distanceFromOrigin |> toString) ]
        , mapView model.stateHistory
        , stateHistoryView model.stateHistory
        ]


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
