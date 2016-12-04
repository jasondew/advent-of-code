module Room exposing (Room, isReal, parse)

import Char
import List.Extra


type alias Room =
    { encryptedName : String
    , name : String
    , sectorID : Int
    , checksum : String
    , computedChecksum : String
    }


isReal : Room -> Bool
isReal room =
    room.checksum == room.computedChecksum


computeChecksum : String -> String
computeChecksum string =
    withoutDashes string
        |> topFiveLetters
        |> String.fromList


withoutDashes : String -> String
withoutDashes =
    String.filter (\char -> not <| char == '-')


topFiveLetters : String -> List Char
topFiveLetters string =
    string
        |> String.toList
        |> List.sort
        |> List.Extra.group
        |> List.sortBy (List.length >> negate)
        |> List.take 5
        |> List.map (List.head >> Maybe.withDefault '?')


decryptName : Int -> String -> String
decryptName shift string =
    string
        |> String.split "-"
        |> List.map (decrypt shift)
        |> String.join " "


decrypt : Int -> String -> String
decrypt shift string =
    string
        |> String.map (shiftChar shift)


shiftChar : Int -> Char -> Char
shiftChar shift char =
    let
        baseCode =
            Char.toCode 'a'

        alphabetLength =
            26
    in
        char
            |> Char.toCode
            |> (\code -> ((code - baseCode + shift) % alphabetLength) + baseCode)
            |> Char.fromCode


parse : String -> Room
parse string =
    case string |> String.split "-" |> List.reverse of
        idAndChecksum :: rest ->
            let
                encryptedName =
                    rest |> List.reverse |> String.join "-"

                sectorIDStringAndChecksum =
                    idAndChecksum |> String.dropRight 1 |> String.split "["

                computedChecksum =
                    computeChecksum encryptedName
            in
                case sectorIDStringAndChecksum of
                    sectorIDString :: checksum :: [] ->
                        let
                            sectorID =
                                parseInt sectorIDString

                            name =
                                decryptName sectorID encryptedName
                        in
                            Room encryptedName name sectorID checksum computedChecksum

                    _ ->
                        invalid

        _ ->
            invalid


invalid : Room
invalid =
    Room "invalid" "invalid" 0 "invalid" "invalid"


parseInt : String -> Int
parseInt string =
    case String.toInt string of
        Ok int ->
            int

        Err _ ->
            0
