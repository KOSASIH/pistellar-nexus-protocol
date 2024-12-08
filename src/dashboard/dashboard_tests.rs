#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_index() {
        let state = AppState {
            user_count: Mutex::new(5),
        };

        let app = test::init_service(App::new().data(state).route("/", web::get().to(index))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: serde_json::Value = test::read_response_json(&app, req).await;

        assert_eq!(resp["message"], "Welcome to the Dashboard!");
        assert_eq!(resp["user_count"], 5);
    }

    #[actix_web::test]
    async fn test_increment_user_count() {
        let state = AppState {
            user_count: Mutex::new(5),
        };

        let app = test::init_service(App::new().data(state).route("/increment", web::post().to(increment_user_count))).await;
        let req = test::TestRequest::post().uri("/increment").to_request();
        let resp: serde_json::Value = test::read_response_json(&app, req).await;

        assert_eq!(resp["user_count"], 6);
    }
}
