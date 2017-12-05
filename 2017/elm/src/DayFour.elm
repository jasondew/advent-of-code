module DayFour exposing (partOne, partTwo)

import List.Extra


partOne : String -> Int
partOne input =
    input
        |> String.trim
        |> String.lines
        |> List.filter uniqueWords
        |> List.length


partTwo : String -> Int
partTwo input =
    input
        |> String.trim
        |> String.lines
        |> List.filter withoutAnagrams
        |> List.length


uniqueWords : String -> Bool
uniqueWords passphrase =
    let
        allWords =
            String.words passphrase

        uniqueWords =
            List.Extra.unique allWords
    in
        List.length allWords == List.length uniqueWords


withoutAnagrams : String -> Bool
withoutAnagrams passphrase =
    let
        allWords =
            String.words passphrase
                |> List.map (String.toList >> List.sort >> String.fromList)

        uniqueWords =
            List.Extra.unique allWords
    in
        List.length allWords == List.length uniqueWords
