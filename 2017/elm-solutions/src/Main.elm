module Main exposing (..)

import DayOne
import DayTwo
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)


---- MODEL ----


type alias Model =
    { dayOneInput : String
    , dayTwoInput : String
    }


init : ( Model, Cmd Msg )
init =
    ( Model
        "951484596541141557316984781494999179679767747627132447513171626424561779662873157761442952212296685573452311263445163233493199211387838461594635666699422982947782623317333683978438123261326863959719777179228599319321138948466562743761584836184512984131635354116264899181952748224523953976485816295227945792555726121913344959454458829485471174415775278865324142733339789878929596275998341778873889585819916457474773252249179366599951454182657225576277834669222982366884688565754691273745959468648957498511326215934353963981471593984617554514519623785326888374742147318993423214834751785956958395133486656388454552769722562524415715913869946325551396638593398729938526424994348267935153555851552287223313383583669912941364344694725478258297498969517632881187394141593479818536194597976519254215932257653777455227477617957833273463216593642394215275314734914719726618923177918342664351954252667253233858814365351722938716621544226598956257753212248859258351363174782742336961425325381561575992352415514168782816173861148859478285339529151631429536819286498721812323861771638574344416879476255929929157912984151742613268754779685396125954595318134933366626594498249956388771723777242772654678448815844555372892574747735672368299826548254744359377667294764559334659523233146587568261116253155189394188696831691284711264872914348961888253386971994431352474717376878745948769171243242621219912378731755544387249443997382399714738351857752329367997665166956467544459817582915478514486541453932175598413554259672117364863112592515988922747164842668361925135551248923449968328385889877512156952725198691746951431443497496455761516486573476185321748523644283494181119399874324683922393547682851931435931276267766772798261563117954648576421741384823494187895272582575669685279986988357796138794326125852772995446355723211161523161886222562853546488411563473998633847953246787557146187696947831335722888918172961256498971868946237299523474841983527391489962357196433927251798764362493965894995592683296651874787384247326643886774966828657393717626591578321174832222434128817871765347278152799425565633521152643686221411129463425496425385516719682884157452772141585743166647191938727971366274357874252166721759"
        """
62 1649 1731 76 51 1295 349 719 52 1984 2015 2171 981 1809 181 1715
161 99 1506 1658 84 78 533 242 1685 86 107 1548 670 960 1641 610
95 2420 2404 2293 542 2107 2198 121 109 209 2759 1373 1446 905 1837 111
552 186 751 527 696 164 114 530 558 307 252 200 481 142 205 479
581 1344 994 1413 120 112 656 1315 1249 193 1411 1280 110 103 74 1007
2536 5252 159 179 4701 1264 1400 2313 4237 161 142 4336 1061 3987 2268 4669
3270 1026 381 185 293 3520 1705 1610 3302 628 3420 524 3172 244 295 39
4142 1835 4137 3821 3730 2094 468 141 150 3982 147 4271 1741 2039 4410 179
1796 83 2039 1252 84 1641 2165 1218 1936 335 1807 2268 66 102 1977 2445
96 65 201 275 257 282 233 60 57 200 216 134 72 105 81 212
3218 5576 5616 5253 178 3317 6147 5973 2424 274 4878 234 200 4781 5372 276
4171 2436 134 3705 3831 3952 2603 115 660 125 610 152 4517 587 1554 619
2970 128 2877 1565 1001 167 254 2672 59 473 2086 181 1305 162 1663 2918
271 348 229 278 981 1785 2290 516 473 2037 737 2291 2521 1494 1121 244
2208 2236 1451 621 1937 1952 865 61 1934 49 1510 50 1767 59 194 1344
94 2312 2397 333 1192 106 2713 2351 2650 2663 703 157 89 510 1824 125
"""
    , Cmd.none
    )



---- UPDATE ----


type Msg
    = UpdateDayOneInput String
    | UpdateDayTwoInput String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateDayOneInput input ->
            ( { model | dayOneInput = input }, Cmd.none )

        UpdateDayTwoInput input ->
            ( { model | dayTwoInput = input }, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view model =
    div
        []
        [ dayOneView model
        , dayTwoView model
        ]


dayOneView : Model -> Html Msg
dayOneView model =
    cardView
        [ text "Day 1: Inverse Captcha" ]
        [ input
            [ onInput <| UpdateDayOneInput
            , class "form-control"
            , value model.dayOneInput
            ]
            []
        , solutionView "Part 1:" <| DayOne.partOne model.dayOneInput
        , solutionView "Part 2:" <| DayOne.partTwo model.dayOneInput
        ]


dayTwoView : Model -> Html Msg
dayTwoView model =
    cardView
        [ text "Day 2: Inverse Captcha" ]
        [ input
            [ onInput <| UpdateDayTwoInput
            , class "form-control"
            , value model.dayTwoInput
            ]
            []
        , solutionView "Part 1:" <| DayTwo.partOne model.dayTwoInput
        , solutionView "Part 2:" <| DayTwo.partTwo model.dayTwoInput
        ]


solutionView : String -> a -> Html Msg
solutionView title result =
    p
        []
        [ label [] [ text title ]
        , code [] [ text <| toString result ]
        ]


cardView : List (Html Msg) -> List (Html Msg) -> Html Msg
cardView header content =
    div
        [ class "card mb-3" ]
        [ h4 [ class "card-header" ] header
        , div [ class "card-body" ] content
        ]



---- PROGRAM ----


main : Program Never Model Msg
main =
    Html.program
        { view = view
        , init = init
        , update = update
        , subscriptions = always Sub.none
        }
