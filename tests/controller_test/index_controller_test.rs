use std::sync::{Arc, Mutex};

use actix_web::{App, test, web};

use sqlx_user_crud::{AppState, controller};

use super::init_db_context;

#[actix_rt::test]
async fn status_returns_ok_and_message() -> () {
    let db_context = init_db_context().await;

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_index_controller),
    )
        .await;

    let req = test::TestRequest::get().uri("/status").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
