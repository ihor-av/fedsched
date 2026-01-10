module Main exposing (main)

import Browser
import Html exposing (..)
import Html.Attributes exposing (value)
import Html.Events exposing (onInput)
import Http
import Json.Decode
import Json.Encode as Encode



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


type alias Model =
    { input : String
    , eventName : String
    }


init : () -> ( Model, Cmd Msg )
init () =
    ( Model "" "", Cmd.none )


type Msg
    = ChangedInput String
    | GotEvent (Result Http.Error String)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangedInput input ->
            ( { model | input = input }, fetchEvent input )

        GotEvent resp ->
            case resp of
                Ok name ->
                    ( { model | eventName = name }, Cmd.none )

                Err _ ->
                    ( { model | eventName = "Error fetching event" }, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


view : Model -> Html Msg
view model =
    div []
        [ input [ value model.input, onInput ChangedInput, Html.Attributes.placeholder "Enter Event ID" ] []
        , div [] [ text ("Event Name: " ++ model.eventName) ]
        ]



-- HTTP


fetchEvent : String -> Cmd Msg
fetchEvent id =
    Http.post { url = "http://localhost:8000/graphql", body = Http.jsonBody (encodeQuery id), expect = Http.expectJson GotEvent eventDecoder }


encodeQuery : String -> Encode.Value
encodeQuery id =
    let
        queryString =
            """
                query {
                  event(id: \"""" ++ id ++ """") {
                    name
                  }
                }
            """
    in
    Encode.object [ ( "query", Encode.string queryString ) ]


eventDecoder : Json.Decode.Decoder String
eventDecoder =
    Json.Decode.at [ "data", "event", "name" ] Json.Decode.string
