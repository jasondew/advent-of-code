module Triangles exposing (main)

import Html exposing (..)
import Html.Attributes exposing (value)
import Html.Events exposing (onInput)
import List.Extra


type alias Triplet =
    ( Int, Int, Int )


type alias Model =
    { lengthTriplets : List Triplet
    , verticalLengthTriplets : List Triplet
    , triangleCount : Int
    , verticalTriangleCount : Int
    }


type Msg
    = NewLengthTriplets String


init : ( Model, Cmd Msg )
init =
    ( Model [] [] 0 0, Cmd.none )


parseLengthTriplets : String -> List Triplet
parseLengthTriplets string =
    string
        |> String.lines
        |> List.map parseTriplet


parseTriplet : String -> Triplet
parseTriplet string =
    case string |> String.words |> List.map parseInt of
        a :: b :: c :: [] ->
            ( a, b, c )

        _ ->
            ( 0, 0, 0 )


parseInt : String -> Int
parseInt string =
    case String.toInt string of
        Ok int ->
            int

        Err _ ->
            0


convertToVerticalLengthTriplets : List Triplet -> List Triplet
convertToVerticalLengthTriplets triplets =
    let
        ( xs, ys, zs ) =
            List.foldl
                (\( x, y, z ) ( xs, ys, zs ) ->
                    ( x :: xs, y :: ys, z :: zs )
                )
                ( [], [], [] )
                triplets

        allValues =
            xs
                |> List.append ys
                |> List.append zs
                |> List.reverse
    in
        allValues
            |> List.Extra.groupsOf 3
            |> List.map
                (\group ->
                    case group of
                        a :: b :: c :: [] ->
                            ( a, b, c )

                        _ ->
                            ( 0, 0, 0 )
                )


validTriangleLengths : List Triplet -> Int
validTriangleLengths triplets =
    List.foldl
        (\triplet count ->
            if validTriangle triplet then
                count + 1
            else
                count
        )
        0
        triplets


validTriangle : Triplet -> Bool
validTriangle ( a, b, c ) =
    a + b > c && b + c > a && a + c > b


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewLengthTriplets string ->
            let
                lengthTriplets =
                    parseLengthTriplets string

                verticalLengthTriplets =
                    convertToVerticalLengthTriplets lengthTriplets
            in
                { model
                    | lengthTriplets = lengthTriplets
                    , triangleCount = validTriangleLengths lengthTriplets
                    , verticalLengthTriplets = verticalLengthTriplets
                    , verticalTriangleCount = validTriangleLengths verticalLengthTriplets
                }
                    ! []


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 3" ]
        , div
            []
            [ label [] [ text "Input: " ]
            , textarea [ onInput NewLengthTriplets ] []
            ]
        , h3 [] [ text <| "Valid horizontal triangle count: " ++ (toString model.triangleCount) ]
        , h3 [] [ text <| "Valid vertical triangle count: " ++ (toString model.verticalTriangleCount) ]
        , h4 [] [ text "Horizontal Triplets" ]
        , tableView model.lengthTriplets
        , h4 [] [ text "Vertical Triplets" ]
        , tableView model.verticalLengthTriplets
        ]


tableView : List Triplet -> Html Msg
tableView lengthTriplets =
    table
        []
        [ thead
            []
            [ th [] [ text "Side 1" ]
            , th [] [ text "Side 2" ]
            , th [] [ text "Side 3" ]
            , th [] [ text "Valid?" ]
            ]
        , tbody [] <| List.map tripletView lengthTriplets
        ]


tripletView : Triplet -> Html Msg
tripletView ( a, b, c ) =
    tr
        []
        [ td [] [ text <| toString a ]
        , td [] [ text <| toString b ]
        , td [] [ text <| toString c ]
        , td [] [ text <| toString <| validTriangle ( a, b, c ) ]
        ]


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
