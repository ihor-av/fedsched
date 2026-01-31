DEFINE TABLE {{ table_name }} SCHEMAFULL;
{% for field_name_and_constraint in field_names_and_constraints %}

    {% let name       = field_name_and_constraint.0 %}
    {% let constraint = field_name_and_constraint.1 %}

    DEFINE FIELD {{ name }} ON TABLE {{ table_name }} TYPE {% match constraint %}
        {% when  FieldConstraint::Text with { regex } %}
            string ASSERT $value = /{{ regex }}/;
        {% when  FieldConstraint::Range with { min, max } %}
            number ASSERT $value >= {{ min }} AND $value <= {{ max }};
        {% when  FieldConstraint::Datetime %}
            datetime;
    {% endmatch %}
{% endfor %}
