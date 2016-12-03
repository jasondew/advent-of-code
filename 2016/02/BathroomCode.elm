module BathroomCode exposing (main)

import Html exposing (..)
import Html.Attributes exposing (value)
import Html.Events exposing (onInput)
import Json.Decode as Json
import BathroomCodeDecoder exposing (..)


type alias Model =
    { instructions : String
    , code : String
    , keypad : Keypad
    }


type Msg
    = NewInstructions String
    | NewKeypad String


init : ( Model, Cmd Msg )
init =
    ( Model "" "" Simple, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewInstructions instructions ->
            { model
                | instructions = instructions
                , code = decode model.keypad instructions
            }
                ! []

        NewKeypad keypadString ->
            let
                newKeypad =
                    if keypadString == "simple" then
                        Simple
                    else
                        Advanced
            in
                { model
                    | keypad = newKeypad
                    , code = decode newKeypad model.instructions
                }
                    ! []


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 2" ]
        , div
            []
            [ label [] [ text "Keypad: " ]
            , select
                [ onChange NewKeypad ]
                [ option [ value "simple" ] [ text "Simple" ]
                , option [ value "advanced" ] [ text "Advanced" ]
                ]
            ]
        , div
            []
            [ label [] [ text "Instructions: " ]
            , textarea [ onInput NewInstructions ] []
            ]
        , h3 [] [ text <| "Code: " ++ (model.code) ]
        ]


onChange : (String -> Msg) -> Attribute Msg
onChange handler =
    Html.Events.on "change" <| Json.map handler <| Html.Events.targetValue


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
