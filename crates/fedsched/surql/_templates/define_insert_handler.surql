/*
INSERT HANDLER
    the purpose of this function
    is to insert row into arbitrary table
    and return standart `json`
*/

DEFINE FUNCTION fn::insert::{{table_name}}( $data: object ) -> object {
    {% for (name, _) in field_names_and_constraints %}
        LET ${{ name }} = fn::unwrap($data.{{ name }});
    {% endfor %}

    LET $res = CREATE {{table_name}} SET
        {% for (name, constraint) in field_names_and_constraints %}
            {{ name }} = {% match constraint %}
                {% when FieldConstraint::Datetime %}
                    <datetime>${{ name }}
                {% else %}
                    ${{ name }}
            {% endmatch %}{% if !loop.last %} , {% endif %}
        {% endfor %}
        ;
    ;
    RETURN {
        id: <string>$res[0].id,
        {% for (name, constraint) in field_names_and_constraints %}
            {{ name }} : {% match constraint %}
                {% when FieldConstraint::Datetime %}
                    <string>$res[0].{{ name }}
                {% else %}
                    $res[0].{{ name }}
            {% endmatch %}{% if !loop.last %} , {% endif %}
        {% endfor %}
    };
}
