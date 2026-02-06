use crate::{
    config::{FieldConfig, TableGroup},
    error::FedschedResult,
    surql_templates::{DefineField, DefineInsertHandler, EventSelectonByDaterange},
};
use askama::Template;
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
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("fedsched").use_db("fedsched").await?;
        db.query(main_surql).await?;
        let grouped_cfgs = TableGroup::try_group(cfgs)?;
        for group in grouped_cfgs {
            let field_surql = DefineField::from(&group).render()?;
            db.query(field_surql).await?;
            let insert_handler_surql = DefineInsertHandler::from(&group).render()?;
            db.query(insert_handler_surql).await?;
            let select_handler_surql = EventSelectonByDaterange::from(&group).render()?;
            db.query(select_handler_surql).await?;
        }
        Ok(Self { db })
    }
}
