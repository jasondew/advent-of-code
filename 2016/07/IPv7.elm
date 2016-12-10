port module IPv7 exposing (main)

import Html exposing (..)
import Html.Events exposing (onInput)
import List.Extra
import Regex


type alias Model =
    { input : List String
    , tlsCount : Int
    , sslCount : Int
    }


type Msg
    = NewInput String


initialInput : String
initialInput =
    "abba[mnop]qrst\nabcd[bddb]xyyx\naaaa[qwer]tyui\nioxxoj[asdfgh]zxcvbn\naba[bab]xyz\nxyx[xyx]xyx\naaa[kek]eke\nzazbz[bzb]cdb\nkonztznxgyjsvynvl[fjejsdhfcynplct]fdnapcnuzqsgwxbdulv[fmxdbdjrhtqglsvtwwg]xumwevxvrhwrqblhzbh[paxrxvxynvppmwt]znpjdeeqlribvbqm"


init : ( Model, Cmd Msg )
init =
    update (NewInput initialInput) (Model [] 0 0)


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        NewInput input ->
            let
                newInput =
                    String.lines input
            in
                { model
                    | input = newInput
                    , tlsCount = count isTLS newInput
                    , sslCount = count isSSL newInput
                }
                    ! []


count : (String -> Bool) -> List String -> Int
count isCounted ips =
    List.foldl
        (\ip count ->
            if isCounted ip then
                count + 1
            else
                count
        )
        0
        ips


isTLS : String -> Bool
isTLS ip =
    let
        bracketedExpressions =
            Regex.find Regex.All hypernetRegex ip |> List.map .match

        outsideExpression =
            Regex.replace Regex.All hypernetRegex (always "-") ip
    in
        (not <| List.any isLengthFourPalindrome bracketedExpressions)
            && (isLengthFourPalindrome outsideExpression)


isLengthFourPalindrome : String -> Bool
isLengthFourPalindrome =
    Regex.contains (Regex.regex "(.)(?!\\1)(.)\\2\\1")


hypernetRegex : Regex.Regex
hypernetRegex =
    Regex.regex "\\[[^\\]]+\\]"


isSSL : String -> Bool
isSSL ip =
    let
        hypernets =
            Regex.find Regex.All hypernetRegex ip |> List.map (.match >> String.dropLeft 1 >> String.dropRight 1)

        supernet =
            Regex.replace Regex.All hypernetRegex (always "--") ip

        babs =
            hypernets
                |> List.map findLengthThreePalindromes
                |> List.concat

        abas =
            List.map (\bab -> (String.dropLeft 1 bab) ++ (String.slice 1 2 bab)) babs
    in
        List.any (\aba -> String.contains aba supernet) abas


findLengthThreePalindromes : String -> List String
findLengthThreePalindromes string =
    let
        chars =
            String.toList string
    in
        List.Extra.zip3 chars (List.drop 1 chars) (List.drop 2 chars)
            |> List.filter (\( a, b, c ) -> a == c && a /= b)
            |> List.map (\( a, b, _ ) -> String.fromList [ a, b, a ])


view : Model -> Html Msg
view model =
    div
        []
        [ h1 [] [ text "Advent of Code 2016 — Day 7" ]
        , div
            []
            [ div
                []
                [ label [] [ text "Input: " ]
                , textarea [ onInput NewInput ] [ text <| String.join "\n" model.input ]
                ]
            ]
        , h3 [] [ text "TLS Count: ", text <| toString model.tlsCount ]
        , h3 [] [ text "SSL Count: ", text <| toString model.sslCount ]
        , inputView model.input
        ]


inputView : List String -> Html Msg
inputView input =
    table
        []
        [ thead
            []
            [ th [] [ text "IP" ]
            , th [] [ text "TLS?" ]
            , th [] [ text "SSL?" ]
            ]
        , tbody [] <| List.map ipView input
        ]


ipView : String -> Html Msg
ipView ip =
    tr
        []
        [ td [] [ pre [] [ text ip ] ]
        , td [] [ text <| toString (isTLS ip) ]
        , td [] [ text <| toString (isSSL ip) ]
        ]


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
