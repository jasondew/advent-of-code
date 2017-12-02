module DayOne exposing (partOne, partTwo)


type Offset
    = Next
    | Half


partOne : String -> Int
partOne code =
    repeatingSum Next code


partTwo : String -> Int
partTwo code =
    repeatingSum Half code


repeatingSum : Offset -> String -> Int
repeatingSum offset code =
    repeatingSumHelper code 0 (String.length code) offset 0


repeatingSumHelper : String -> Int -> Int -> Offset -> Int -> Int
repeatingSumHelper code index length offset sum =
    if index >= length then
        sum
    else
        let
            char =
                String.slice index (index + 1) code

            newSum =
                if isMatching char code index length offset then
                    sum + toInt char
                else
                    sum
        in
            repeatingSumHelper code (index + 1) length offset newSum


isMatching : String -> String -> Int -> Int -> Offset -> Bool
isMatching char code index length offset =
    let
        nextIndex =
            nextIndexGivenOffset offset length index

        nextChar =
            String.slice nextIndex (nextIndex + 1) code
    in
        char == nextChar


nextIndexGivenOffset : Offset -> Int -> Int -> Int
nextIndexGivenOffset offset length index =
    let
        indexOffset =
            case offset of
                Next ->
                    1

                Half ->
                    length // 2
    in
        (index + indexOffset) % length


toInt : String -> Int
toInt digitString =
    case String.toInt digitString of
        Ok int ->
            int

        Err _ ->
            0
