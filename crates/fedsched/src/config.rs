use std::collections::HashMap;

use tracing::{info, instrument, warn};

/// Configuration for field on table
#[derive(Clone)]
pub struct FieldConfig {
    pub table_name: String,
    pub field_name: String,
    pub constraint: FieldConstraint,
}
#[derive(Clone)]
pub enum FieldConstraint {
    Text { regex: String },
    Range { min: u64, max: u64 },
    Datetime,
}

/// Absraction over per-field configuration
/// to keep all configuration grouped by table
pub struct TableGroup<'a> {
    pub table_name: &'a str,
    pub fields: Vec<&'a FieldConfig>,
}
impl<'a> TableGroup<'a> {
    pub fn try_group(cfgs: &'a [FieldConfig]) -> crate::error::FedschedResult<Vec<Self>> {
        let mut map: HashMap<&'a str, Vec<&'a FieldConfig>> = HashMap::new();
        for cfg in cfgs {
            map.entry(&cfg.table_name).or_default().push(cfg);
        }
        let groups: Vec<TableGroup<'a>> = map
            .into_iter()
            .map(|(table_name, fields)| TableGroup { table_name, fields })
            .collect();
        Ok(groups)
    }
}

pub fn mandatory_event_table_fields() -> Vec<FieldConfig> {
    use FieldConstraint::*;
    vec![
        FieldConfig {
            table_name: "event".to_owned(),
            field_name: "event_name".to_owned(),
            constraint: Text {
                regex: ".*".to_owned(),
            },
        },
        FieldConfig {
            table_name: "event".to_owned(),
            field_name: "event_startdate".to_owned(),
            constraint: Datetime,
        },
        FieldConfig {
            table_name: "event".to_owned(),
            field_name: "event_enddate".to_owned(),
            constraint: Datetime,
        },
    ]
}

/// Function to merge lists of configs
/// without duplicates.
///
/// 2 rules are followed:
/// - first instance of duplicate always wins
/// - always prefer fields from right (ie. they are _right_)
#[instrument(level = "info", skip(left, right))]
pub(crate) fn smart_config_merge(
    left: Vec<FieldConfig>,
    right: Vec<FieldConfig>,
) -> Vec<FieldConfig> {
    enum Source {
        Left,
        Right,
    }
    let mut map: HashMap<(String, String), (FieldConfig, Source)> = HashMap::new();
    for cfg in left {
        let key = (cfg.table_name.clone(), cfg.field_name.clone());
        if map.contains_key(&key) {
            info!(table_name = %key.0, field_name = %key.1, "Skipping internal duplicates in LEFT");
        } else {
            map.insert(key, (cfg, Source::Left));
        }
    }
    for cfg in right {
        let key = (cfg.table_name.clone(), cfg.field_name.clone());
        if let Some((_, source)) = map.get(&key) {
            match source {
                Source::Left => {
                    warn!(
                        table_name = %key.0,
                        field_name = %key.1,
                        "Overwriting LEFT item with RIGHT item"
                    );
                    map.insert(key, (cfg, Source::Right));
                }
                Source::Right => {
                    info!(table_name = %key.0, field_name = %key.1, "Skipping internal duplicate in RIGHT");
                }
            }
        } else {
            map.insert(key, (cfg, Source::Right));
        }
    }
    map.into_values().map(|(cfg, _)| cfg).collect()
}
