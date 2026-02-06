use async_graphql::dynamic::{
    Field, FieldFuture, FieldValue, InputObject, InputValue, Object, Schema, TypeRef,
};
use serde_json::json;

use crate::{
    config::{FieldConfig, FieldConstraint},
    error::FedschedResult,
    graphql::SchedulerContext,
};

pub fn build_schema(ctx: SchedulerContext, cfgs: Vec<FieldConfig>) -> FedschedResult<Schema> {
    let event_type = build_output_gql_event_object(cfgs.clone());
    let input_type = build_input_gql_event_object(cfgs.clone());
    let query_root = build_query_root();
    let mutation_root = build_mutation_root();

    Ok(Schema::build("Query", Some("Mutation"), None)
        .register(event_type)
        .register(input_type)
        .register(query_root)
        .register(mutation_root)
        .data(ctx)
        .finish()?)
}

fn build_mutation_root() -> async_graphql::dynamic::Object {
    Object::new("Mutation").field(
        Field::new("createEvent", TypeRef::named("Event"), move |ctx| {
            FieldFuture::new(async move {
                let scheduler = ctx.data::<SchedulerContext>()?;
                let db = &scheduler.db;

                let input_val = ctx.args.try_get("input")?;
                let json_data_serde: serde_json::Value = input_val.deserialize()?;

                let mut response = db
                    .query("RETURN fn::insert::event($data)")
                    .bind(("data", json_data_serde))
                    .await?;
                let created_record: Option<serde_json::Value> = response.take(0)?;
                match created_record {
                    Some(record) => Ok(Some(FieldValue::owned_any(record))),
                    None => Err("Failed to create record".into()),
                }
            })
        })
        .argument(InputValue::new("input", TypeRef::named("CreateEventInput"))),
    )
}

fn build_query_root() -> Object {
    Object::new("Query").field(
        Field::new("getEvents", TypeRef::named_list("Event"), |ctx| {
            FieldFuture::new(async move {
                let scheduler = ctx.data::<SchedulerContext>()?;
                let db = &scheduler.db;

                let from_datetime = ctx.args.try_get("from")?.deserialize::<String>()?;
                let to_datetime = ctx.args.try_get("to")?.deserialize::<String>()?;

                let mut response = db
                    .query("RETURN fn::select::event($data)")
                    .bind((
                        "data",
                        json!({
                            "from" : from_datetime,
                            "to" : to_datetime
                        }),
                    ))
                    .await?;
                let records: Vec<serde_json::Value> = response.take(0)?;
                let gql_results: Vec<async_graphql::dynamic::FieldValue> =
                    records.into_iter().map(FieldValue::owned_any).collect();
                Ok(Some(FieldValue::list(gql_results)))
            })
        })
        .argument(InputValue::new("from", TypeRef::named(TypeRef::STRING)))
        .argument(InputValue::new("to", TypeRef::named(TypeRef::STRING))),
    )
}

fn build_output_gql_event_object(cfgs: Vec<FieldConfig>) -> async_graphql::dynamic::Object {
    let mut object = async_graphql::dynamic::Object::new("Event");
    object = object.field(Field::new("id", TypeRef::named(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let parent_value = ctx.parent_value.try_downcast_ref::<serde_json::Value>()?;
            let val = parent_value.get("id").and_then(|v| v.as_str());
            match val {
                Some(id_str) => Ok(Some(FieldValue::value(id_str.to_string()))),
                None => Ok(None),
            }
        })
    }));
    for cfg in cfgs {
        let ty = match cfg.constraint {
            FieldConstraint::Text { .. } => TypeRef::named(TypeRef::STRING),
            FieldConstraint::Range { .. } => TypeRef::named(TypeRef::INT),
            FieldConstraint::Datetime => TypeRef::named(TypeRef::STRING),
        };

        let field_name = cfg.field_name.clone();
        let lookup_key = cfg.field_name.clone();

        let field = Field::new(field_name, ty, move |ctx| {
            let key_for_this_reqwest = lookup_key.clone();
            FieldFuture::new(async move {
                let parent_value = ctx.parent_value.try_downcast_ref::<serde_json::Value>()?;
                let val = parent_value.get(&key_for_this_reqwest).cloned();
                match val {
                    Some(val) => {
                        let gql_value = async_graphql::to_value(val)?;
                        Ok(Some(FieldValue::from(gql_value)))
                    }
                    None => Ok(None),
                }
            })
        });

        object = object.field(field);
    }
    object
}

fn build_input_gql_event_object(cfgs: Vec<FieldConfig>) -> InputObject {
    let mut input = InputObject::new("CreateEventInput");
    for cfg in cfgs {
        let ty = match cfg.constraint {
            FieldConstraint::Text { .. } => TypeRef::named(TypeRef::STRING),
            FieldConstraint::Range { .. } => TypeRef::named(TypeRef::INT),
            FieldConstraint::Datetime => TypeRef::named(TypeRef::STRING),
        };
        let field_name = cfg.field_name;
        let input_val = InputValue::new(field_name, ty);

        input = input.field(input_val);
    }
    input
}
