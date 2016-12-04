module Room exposing (Room, isReal, parseRoom)


type alias Room =
    { encryptedName : String
    , sectorID : Int
    , checksum : String
    }


isReal : Room -> Bool
isReal room =
    False


parseRoom : String -> Room
parseRoom string =
    Room "name" 123 "checksum"


parseInt : String -> Int
parseInt string =
    case String.toInt string of
        Ok int ->
            int

        Err _ ->
            0
