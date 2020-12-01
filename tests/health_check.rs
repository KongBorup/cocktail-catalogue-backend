use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = cocktail_catalogue_backend::server::start(listener).expect("failed to start server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}