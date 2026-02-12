use askama::Template;

use crate::config::{FieldConstraint, TableGroup};

#[derive(Template)]
#[template(path = "define_field.surql", escape = "none")]
pub(crate) struct DefineField<'a> {
    pub(crate) table_name: &'a str,
    pub(crate) field_names_and_constraints: Vec<(&'a str, &'a FieldConstraint)>,
}
impl<'a> From<&TableGroup<'a>> for DefineField<'a> {
    fn from(value: &TableGroup<'a>) -> Self {
        let table_name = value.table_name;

        let field_names_and_constraints: Vec<(&'a str, &'a FieldConstraint)> = value
            .fields
            .iter()
            .map(|cfg| (cfg.field_name.as_str(), &cfg.constraint))
            .collect();

        Self {
            table_name,
            field_names_and_constraints,
        }
    }
}

#[derive(Template)]
#[template(path = "define_insert_handler.surql", escape = "none")]
pub(crate) struct DefineInsertHandler<'a> {
    pub(crate) table_name: &'a str,
    pub(crate) field_names_and_constraints: Vec<(&'a str, &'a FieldConstraint)>,
}
impl<'a> From<&TableGroup<'a>> for DefineInsertHandler<'a> {
    fn from(value: &TableGroup<'a>) -> Self {
        let table_name = value.table_name;

        let field_names_and_constraints: Vec<(&'a str, &'a FieldConstraint)> = value
            .fields
            .iter()
            .map(|cfg| (cfg.field_name.as_str(), &cfg.constraint))
            .collect();

        Self {
            table_name,
            field_names_and_constraints,
        }
    }
}
#[derive(Template)]
#[template(path = "event_selection_by_daterange.surql", escape = "none")]
pub(crate) struct EventSelectonByDaterange<'a> {
    pub(crate) field_names_and_constraints: Vec<(&'a str, &'a FieldConstraint)>,
}
impl<'a> From<&TableGroup<'a>> for EventSelectonByDaterange<'a> {
    fn from(value: &TableGroup<'a>) -> Self {
        let field_names_and_constraints: Vec<(&'a str, &'a FieldConstraint)> = value
            .fields
            .iter()
            .map(|cfg| (cfg.field_name.as_str(), &cfg.constraint))
            .collect();

        Self {
            field_names_and_constraints,
        }
    }
}
