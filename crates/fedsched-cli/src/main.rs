use fedsched::prelude::*;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

fn possible_user_config() -> Vec<FieldConfig> {
    use FieldConstraint::*;
    vec![
        FieldConfig {
            table_name: "event".to_owned(),
            field_name: "name_of_the_speaker".to_owned(),
            constraint: Text {
                regex: ".*".to_owned(),
            },
        },
        FieldConfig {
            table_name: "event".to_owned(),
            field_name: "dog_of_the_speaker".to_owned(),
            constraint: Text {
                regex: ".*".to_owned(),
            },
        },
        FieldConfig {
            table_name: "event".to_owned(),
            field_name: "birthday_of_speaker".to_owned(),
            constraint: Datetime,
        },
    ]
}

#[tokio::main]
async fn main() -> FedschedResult<()> {
    let cfgs = possible_user_config();
    let router = setup_scheduler(cfgs).await?;

    let router = if cfg!(debug_assertions) {
        router.layer(CorsLayer::permissive())
    } else {
        router
    };

    println!("Router running on http://localhost:8000/");
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), router)
        .await
        .unwrap();
    Ok(())
}
