port module SecurityCode exposing (main)

import Html exposing (..)
import Html.Attributes exposing (value)
import Html.Events exposing (onInput, onClick)
import Array exposing (Array)


port hash : String -> Cmd msg


port hashed : (String -> msg) -> Sub msg


type Method
    = Simple
    | WarGames


type alias Password =
    Array String


type alias Model =
    { doorID : String
    , method : Method
    , password : Password
    , index : Int
    , digitsFound : Int
    }


type Msg
    = NewDoorID String
    | ToggleMethod
    | FindPassword
    | HashReturned String


initialDoorID : String
initialDoorID =
    "reyedfim"


init : ( Model, Cmd Msg )
init =
    update (NewDoorID initialDoorID) (Model "" WarGames initialPassword 0 0)


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewDoorID doorID ->
            { model | doorID = doorID } ! []

        ToggleMethod ->
            case model.method of
                Simple ->
                    { model | method = WarGames } ! []

                WarGames ->
                    { model | method = Simple } ! []

        FindPassword ->
            let
                newModel =
                    { model | index = 0, password = initialPassword, digitsFound = 0 }
            in
                newModel ! [ computeHash newModel ]

        HashReturned hash ->
            if String.left 5 hash == "00000" then
                let
                    maybeUpdatedPassword =
                        updatePassword model hash

                    indexIncrementedModel =
                        { model | index = model.index + 1 }
                in
                    case maybeUpdatedPassword of
                        Just updatedPassword ->
                            let
                                newModel =
                                    { indexIncrementedModel | password = updatedPassword, digitsFound = model.digitsFound + 1 }
                            in
                                if newModel.digitsFound == passwordSize then
                                    newModel ! []
                                else
                                    newModel ! [ computeHash newModel ]

                        Nothing ->
                            indexIncrementedModel ! [ computeHash indexIncrementedModel ]
            else
                let
                    newModel =
                        { model | index = model.index + 1 }
                in
                    newModel ! [ computeHash newModel ]


passwordSize : Int
passwordSize =
    8


initialPassword : Password
initialPassword =
    Array.repeat passwordSize initialPasswordCharacter


initialPasswordCharacter : String
initialPasswordCharacter =
    "-"


updatePassword : Model -> String -> Maybe Password
updatePassword model hash =
    let
        ( maybePosition, character ) =
            case model.method of
                Simple ->
                    ( Ok <| model.digitsFound, String.slice 5 6 hash )

                WarGames ->
                    ( String.slice 5 6 hash |> String.toInt, String.slice 6 7 hash )
    in
        case maybePosition of
            Ok position ->
                if position < passwordSize && (Array.get position model.password |> Maybe.withDefault initialPasswordCharacter) == initialPasswordCharacter then
                    Just <| Array.set position character model.password
                else
                    Nothing

            Err _ ->
                Nothing


computeHash : Model -> Cmd Msg
computeHash model =
    hash <| model.doorID ++ (toString model.index)


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 5" ]
        , div
            []
            [ div
                []
                [ label [] [ text <| "Method: " ++ (toString model.method) ], button [ onClick ToggleMethod ] [ text "Toggle Method" ] ]
            , div
                []
                [ label [] [ text "Door ID: " ]
                , input [ onInput NewDoorID, value model.doorID ] []
                ]
            , div [] [ button [ onClick FindPassword ] [ text "Find the Password!" ] ]
            ]
        , h3 [] [ text <| "Index: " ++ (toString model.index) ]
        , h3 [] [ text <| "Digits found: " ++ (toString model.digitsFound) ]
        , h3 [] [ text "Password: ", passwordView model.password ]
        ]


passwordView : Password -> Html Msg
passwordView password =
    pre [] [ password |> Array.toList |> String.concat |> text ]


subscriptions : Model -> Sub Msg
subscriptions model =
    hashed HashReturned


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }
