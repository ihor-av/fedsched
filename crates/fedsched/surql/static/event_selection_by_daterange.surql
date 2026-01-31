/*
GET EVENTS
    function to get array of events from date range
*/

DEFINE FUNCTION fn::select::event($data: object) -> array<object> {
    LET $min_startdate = <datetime>fn::unwrap($data.from);
    LET $max_startdate = <datetime>fn::unwrap($data.to);
    RETURN SELECT * FROM event
        WHERE
            event_startdate >= $min_startdate AND
            event_startdate <= $max_startdate;
}
