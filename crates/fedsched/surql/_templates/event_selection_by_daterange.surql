/*
GET EVENTS
    function to get array of events from date range
*/

DEFINE FUNCTION fn::select::event($data: object) -> array<object> {
    LET $min_startdate = <datetime>fn::unwrap($data.from);
    LET $max_startdate = <datetime>fn::unwrap($data.to);
    RETURN SELECT
        <string>id AS id,
        {% for (name, constraint) in field_names_and_constraints %}
            {% match constraint %}
                {% when FieldConstraint::Datetime %}
                    <string>{{ name }} AS {{ name }}
                {% else %}
                    {{ name }}
            {% endmatch %}{% if !loop.last %} , {% endif %}
        {% endfor %}
        FROM event
        WHERE
            event_startdate >= $min_startdate AND
            event_startdate <= $max_startdate;
}
