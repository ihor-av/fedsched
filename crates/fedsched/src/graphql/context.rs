use crate::{
    config::{FieldConfig, TableGroup},
    error::FedschedResult,
    surql_templates::{DefineField, DefineInsertHandler},
};
use askama::Template;
use include_dir::include_dir;
use surrealdb::{
    Surreal,
    engine::local::{Db, Mem},
};
#[derive(Clone)]
pub struct SchedulerContext {
    pub(crate) db: Surreal<Db>,
}
impl SchedulerContext {
    pub(crate) async fn build_ctx_from_cfgs(cfgs: &[FieldConfig]) -> FedschedResult<Self> {
        let main_surql = include_str!("../../surql/utils.surql");
        let static_surql: String = include_dir!("$CARGO_MANIFEST_DIR/surql/static")
            .files()
            .filter_map(|file| file.contents_utf8())
            .collect::<Vec<_>>()
            .join("\n");
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("fedsched").use_db("fedsched").await?;
        db.query(main_surql).await?;
        db.query(static_surql).await?;
        let grouped_cfgs = TableGroup::try_group(cfgs)?;
        for group in grouped_cfgs {
            let field_surql = DefineField::from(&group).render()?;
            db.query(field_surql).await?;
            let handler_surql = DefineInsertHandler::from(&group).render()?;
            db.query(handler_surql).await?;
        }
        Ok(Self { db })
    }
}
