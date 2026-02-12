module Main exposing (main)

import Browser
import Date exposing (Date)
import Element exposing (alignLeft, alignRight, centerX, centerY, column, el, fill, height, inFront, padding, paragraph, pointer, px, rgba, row, spacing, text, width)
import Element.Background as Background
import Element.Border as Border
import Element.Events exposing (onClick)
import Element.Font as Font exposing (bold, italic)
import Element.Input as Input
import Element.Region as Region
import Html exposing (Html)
import Http
import Iso8601
import Json.Decode as Decode exposing (Decoder, at, field, list, string)
import Json.Encode as Encode
import Task
import Time



-- CUSTOM TYPES


{-| State of application' modal window.
Intended to collect Url of new fedsched instances
-}
type ModalWindowState
    = Closed -- Modal window is closed.
    | CollectingMaybeValidFedschedUrl String -- Modal window is collecting Url for possibly valid fedsched instance.



-- MAIN


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }



-- MODEL


type alias Event =
    { id : String
    , name : String
    , start : Time.Posix
    , end : Time.Posix
    }


type alias TaggedEventList =
    { url : String
    , events : List Event
    }


{-| Data related to valid fedsched instance
For now it only stores url,
In future it might story
-}
type alias FedschedInstance =
    { full_url : String
    }


type Model
    = Loading
    | Loaded LoadedData


{-| TODO: Add field \`trigger\_message\_or\_something\` to this function
-}
pullEvents : { from : Date, to : Date, url : String } -> Cmd Msg
pullEvents data =
    let
        fromStr =
            Date.toIsoString data.from

        toStr =
            Date.toIsoString data.to

        body =
            Http.jsonBody <|
                Encode.object
                    [ ( "query"
                      , Encode.string <|
                            "query GetEvents { getEvents(from: \""
                                ++ fromStr
                                ++ "\", to: \""
                                ++ toStr
                                ++ "\") { id event_name event_startdate event_enddate } }"
                      )
                    ]
    in
    Http.post
        { url = data.url
        , body = body
        , expect =
            Http.expectJson
                (\result ->
                    GotEvents (Result.map (\evs -> { url = data.url, events = evs }) result)
                )
                nestedEvtDecoder
        }


type alias LoadedData =
    { all_events : List TaggedEventList
    , now : Time.Posix
    , zone : Time.Zone
    , date_range :
        { from : Date
        , to : Date
        }
    , modal_window_state : ModalWindowState
    , tracked_fedsched_insts : List FedschedInstance
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Loading, Task.perform AppLoadedInitialInfo getInitialInfo )


getInitialInfo : Task.Task x ( Time.Posix, Time.Zone )
getInitialInfo =
    Task.map2 (\posix zone -> ( posix, zone ))
        Time.now
        Time.here


type Msg
    = AppLoadedInitialInfo ( Time.Posix, Time.Zone ) -- Current time and day has been loaded
    | PullAllEvents -- Order to pull all events has been made
    | PullEventsFromInst String -- Pull events from specific instance
    | DateRangeBeenUpdated Date Date -- The new date range has been provided
    | UserClickedNextWeek -- One week forward
    | UserClickedPrevWeek -- One week backward
    | GotEvents (Result Http.Error TaggedEventList) -- Got events, tagged with src
    | OpenModal -- Open the modal window
    | CloseModal -- Close the modal window
    | ModalCollectingUrl String -- Collecting possibly valid Url


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case model of
        Loading ->
            case msg of
                AppLoadedInitialInfo ( now, zone ) ->
                    let
                        today =
                            Date.fromPosix zone now

                        daysPastMonday =
                            Date.weekdayNumber today - 1

                        thisMonday =
                            Date.add Date.Days -daysPastMonday today

                        thisSunday =
                            Date.add Date.Days 6 thisMonday

                        thisWeek =
                            { from = thisMonday, to = thisSunday }
                    in
                    ( Loaded <| LoadedData [] now zone thisWeek Closed []
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        Loaded data ->
            updateLoaded msg data
                |> updateWith Loaded


updateWith : (data -> Model) -> ( data, Cmd Msg ) -> ( Model, Cmd Msg )
updateWith toModel ( data, cmd ) =
    ( toModel data, cmd )


updateLoaded : Msg -> LoadedData -> ( LoadedData, Cmd Msg )
updateLoaded msg data =
    case msg of
        DateRangeBeenUpdated from_date to_date ->
            let
                oldRange =
                    data.date_range

                newRange =
                    { oldRange | from = from_date, to = to_date }

                urlList =
                    data.tracked_fedsched_insts
            in
            ( { data | date_range = newRange }
            , urlList |> List.map (\inst -> pullEvents { from = from_date, to = to_date, url = inst.full_url }) |> Cmd.batch
            )

        PullAllEvents ->
            let
                from =
                    data.date_range.from

                to =
                    data.date_range.to

                urlList =
                    data.tracked_fedsched_insts
            in
            ( data
            , urlList
                |> List.map (\inst -> pullEvents { from = from, to = to, url = inst.full_url })
                |> Cmd.batch
            )

        GotEvents (Ok taggedList) ->
            let
                -- All new events, we replace all events from url with updated ones
                newEvents =
                    taggedList :: List.filter (\event -> event.url /= taggedList.url) data.all_events

                alreadyTracked =
                    List.any (\inst -> inst.full_url == taggedList.url) data.tracked_fedsched_insts

                newTrackedInsts =
                    if alreadyTracked then
                        data.tracked_fedsched_insts

                    else
                        { full_url = taggedList.url } :: data.tracked_fedsched_insts
            in
            ( { data
                | all_events = newEvents
                , tracked_fedsched_insts = newTrackedInsts
              }
            , Cmd.none
            )

        UserClickedNextWeek ->
            let
                newFrom =
                    Date.add Date.Weeks 1 data.date_range.from

                newTo =
                    Date.add Date.Weeks 1 data.date_range.to

                newDateRange =
                    { from = newFrom, to = newTo }
            in
            ( { data | date_range = newDateRange }, List.map (\grp -> pullEvents { from = newFrom, to = newTo, url = grp.full_url }) data.tracked_fedsched_insts |> Cmd.batch )

        UserClickedPrevWeek ->
            let
                newFrom =
                    Date.add Date.Weeks -1 data.date_range.from

                newTo =
                    Date.add Date.Weeks -1 data.date_range.to

                newDateRange =
                    { from = newFrom, to = newTo }
            in
            ( { data | date_range = newDateRange }, List.map (\grp -> pullEvents { from = newFrom, to = newTo, url = grp.full_url }) data.tracked_fedsched_insts |> Cmd.batch )

        PullEventsFromInst url ->
            ( { data | modal_window_state = Closed }, pullEvents { from = data.date_range.from, to = data.date_range.to, url = url } )

        ModalCollectingUrl url ->
            ( { data | modal_window_state = CollectingMaybeValidFedschedUrl url }, Cmd.none )

        OpenModal ->
            ( { data | modal_window_state = CollectingMaybeValidFedschedUrl "" }, Cmd.none )

        CloseModal ->
            ( { data | modal_window_state = Closed }, Cmd.none )

        _ ->
            ( data, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


view : Model -> Html Msg
view model =
    case model of
        Loading ->
            Element.layout [] <|
                el [ centerX, centerY, Font.bold, Font.size 72 ] <|
                    text "Loading..."

        Loaded data ->
            Element.layout [ width fill, height fill ] <|
                column [ width fill, height fill, inFront (viewModal data.modal_window_state) ]
                    [ viewTopBar data
                    , viewEvents data
                    ]


viewTopBar : LoadedData -> Element.Element Msg
viewTopBar data =
    row [ padding 24, spacing 20, width fill ]
        [ Input.button [ alignLeft ] { onPress = Just OpenModal, label = text "[ADD-SRC]" }
        , Input.button [] { onPress = Just UserClickedPrevWeek, label = text "<-" }
        , el [ Font.bold, Font.size 18 ] <| text <| formatRange data.date_range
        , Input.button [] { onPress = Just UserClickedNextWeek, label = text "->" }
        , Input.button [ alignRight ] { onPress = Just PullAllEvents, label = text "[PULL]" }
        ]


viewEvents : LoadedData -> Element.Element Msg
viewEvents data =
    let
        fmt =
            "MMM d, yyyy"

        posixToDate =
            Date.fromPosix Time.utc
    in
    column [ spacing 20 ] <|
        List.map
            (\group ->
                column [ spacing 10 ] <|
                    [ el [ Region.heading 2, Font.bold ] <|
                        text (group.url ++ " - Count: " ++ String.fromInt (List.length group.events))
                    ]
            )
            data.all_events


viewModal : ModalWindowState -> Element.Element Msg
viewModal modal =
    case modal of
        Closed ->
            Element.none

        CollectingMaybeValidFedschedUrl url ->
            el
                [ width fill
                , height fill
                , Background.color (rgba 0 0 0 0.9)
                , pointer
                ]
            <|
                el [ centerX, centerY, width (px 450), padding 30, Border.rounded 10 ] <|
                    column [ spacing 20, width fill ]
                        [ el [ Font.size 30 ] <| text <| "Add new Fedsched instance"
                        , Input.text [ width fill ]
                            { onChange = ModalCollectingUrl
                            , text = url
                            , placeholder = Just (Input.placeholder [] <| text <| "https://...")
                            , label = Input.labelBelow [] <| text <| "Scheduler URL"
                            }
                        , Input.button [ centerX ] { onPress = Just <| PullEventsFromInst url, label = text "[VALIATE]" }
                        ]


formatRange : { from : Date, to : Date } -> String
formatRange { from, to } =
    let
        fmt =
            "MMM d, yyyy"
    in
    Date.format fmt from ++ " -- " ++ Date.format fmt to


{-| Decode an event from JSON.
Only mandatory fields are decoded.
In future, we will handle custom fields.
-}
eventDecoder : Decoder Event
eventDecoder =
    Decode.map4 Event
        (field "id" string)
        (field "event_name" string)
        (field "event_startdate" Iso8601.decoder)
        (field "event_enddate" Iso8601.decoder)


eventsDecoder : Decoder (List Event)
eventsDecoder =
    list eventDecoder


nestedEvtDecoder : Decoder (List Event)
nestedEvtDecoder =
    at [ "data", "getEvents" ] eventsDecoder
