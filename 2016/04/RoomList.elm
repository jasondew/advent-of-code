module RoomList exposing (main)

import Html exposing (..)
import Html.Attributes exposing (value)
import Html.Events exposing (onInput)
import Room exposing (Room)


type alias Model =
    { rooms : List Room
    , realSectorIDSum : Int
    }


type Msg
    = NewRooms String


init : ( Model, Cmd Msg )
init =
    ( Model [] 0, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewRooms string ->
            let
                newRooms =
                    string
                        |> String.lines
                        |> List.map parseRoom

                newRealSectorIDSum =
                    newRooms
                        |> List.filter isReal
                        |> List.foldl (\room sum -> sum + room.sectorID) 0
            in
                { model
                    | rooms = newRooms
                    , realSectorIDSum = newRealSectorIDSum
                }
                    ! []


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 4" ]
        , div
            []
            [ label [] [ text "Input: " ]
            , textarea [ onInput NewRooms ] []
            ]
        , h3 [] [ text <| "Real room sector ID sum: " ++ (toString model.realSectorIDSum) ]
        , roomsView model.rooms
        ]


roomsView : List Room -> Html Msg
roomsView rooms =
    table
        []
        [ thead
            []
            [ th [] [ text "Encrypted Name" ]
            , th [] [ text "Sector ID" ]
            , th [] [ text "Checksum" ]
            , th [] [ text "Real?" ]
            ]
        , tbody [] <| List.map roomView rooms
        ]


roomView : Room -> Html Msg
roomView room =
    tr
        []
        [ td [] [ text room.encryptedName ]
        , td [] [ text <| toString room.sectorID ]
        , td [] [ text room.checksum ]
        , td [] [ text <| toString (isReal room) ]
        ]


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
