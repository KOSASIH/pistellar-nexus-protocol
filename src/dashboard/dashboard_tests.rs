#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, web};
    use std::sync::Mutex;

    #[derive(Clone)]
    struct AppState {
        user_count: Mutex<u32>,
    }

    async fn index(state: web::Data<AppState>) -> impl actix_web::Responder {
        let user_count = state.user_count.lock().unwrap();
        web::Json(serde_json::json!({
            "message": "Welcome to the Dashboard!",
            "user_count": *user_count,
        }))
    }

    async fn increment_user_count(state: web::Data<AppState>) -> impl actix_web::Responder {
        let mut user_count = state.user_count.lock().unwrap();
        *user_count += 1;
        web::Json(serde_json::json!({
            "user_count": *user_count,
        }))
    }

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

    #[actix_web::test]
    async fn test_increment_user_count_concurrently() {
        let state = AppState {
            user_count: Mutex::new(5),
        };

        let app = test::init_service(App::new().data(state).route("/increment", web::post().to(increment_user_count))).await;

        let mut tasks = vec![];
        for _ in 0..10 {
            let req = test::TestRequest::post().uri("/increment").to_request();
            let app_clone = app.clone();
            tasks.push(tokio::spawn(async move {
                test::read_response_json(&app_clone, req).await
            }));
        }

        let responses: Vec<serde_json::Value> = futures::future::join_all(tasks).await.into_iter().filter_map(Result::ok).collect();
        
        // Check that all responses have the correct user count
        for resp in responses {
            assert_eq!(resp["user_count"], 6); // Assuming the initial count was 5 and we incremented 10 times
        }
    }
                                                     }
