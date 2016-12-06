port module SantaComm exposing (main)

import Html exposing (..)
import Html.Attributes exposing (value)
import Html.Events exposing (onInput, onClick)
import List.Extra


type Method
    = MostCommon
    | LeastCommon


type alias Model =
    { input : String
    , message : String
    , method : Method
    }


type Msg
    = NewInput String
    | ToggleMethod


initialInput : String
initialInput =
    "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar"


init : ( Model, Cmd Msg )
init =
    update (NewInput initialInput) (Model "" "" MostCommon)


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewInput input ->
            { model | input = input, message = decode model.method input } ! []

        ToggleMethod ->
            case model.method of
                MostCommon ->
                    { model | method = LeastCommon, message = decode LeastCommon model.input } ! []

                LeastCommon ->
                    { model | method = MostCommon, message = decode MostCommon model.input } ! []


decode : Method -> String -> String
decode method input =
    input
        |> String.lines
        |> List.map String.toList
        |> List.Extra.transpose
        |> List.map (chooseLetter method)
        |> String.fromList


chooseLetter : Method -> List Char -> Char
chooseLetter method letters =
    letters
        |> List.sort
        |> List.Extra.group
        |> List.sortBy List.length
        |> (if method == MostCommon then
                List.Extra.last
            else
                List.head
           )
        |> Maybe.withDefault [ '-' ]
        |> List.Extra.last
        |> Maybe.withDefault '-'


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 6" ]
        , div
            []
            [ div
                []
                [ label [] [ text <| "Method: " ++ (toString model.method) ]
                , button [ onClick ToggleMethod ] [ text "Toggle Method" ]
                ]
            , div
                []
                [ label [] [ text "Input: " ]
                , textarea [ onInput NewInput ] [ text model.input ]
                ]
            ]
        , h3 [] [ text "Message: ", text model.message ]
        ]


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
