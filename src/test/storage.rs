use reqwest::{Client, StatusCode};
use reqwest::multipart::{Form, Part};
use serde_json::Value;

const TOKEN: &str = "7k6AHruhNFK8fMh69qqC9y";
const CONTAINER: &str = "lms";

#[test]
pub async fn uploaded() {
    let file = std::fs::read("./src/main.rs").unwrap();
    let form = Form::new()
        .part("files", Part::bytes(file.clone()).file_name("storage1").mime_str("application/octet-stream").unwrap())
        .part("files", Part::bytes(file).file_name("storage2").mime_str("application/octet-stream").unwrap());

    let client = Client::new()
        .post(format!("http://nightmare-storage-app:8000/api/v1/storage/{CONTAINER}"))
        .bearer_auth(TOKEN)
        .multipart(form);

    match client.send().await {
        Err(e) => panic!("{}", e),
        Ok(response) => {
            let status = response.status();
            let body = response.bytes().await.unwrap();

            assert!(status == StatusCode::CREATED, "assert status code created, {:?}", body);

            let response = serde_json::from_slice::<Value>(&body);

            assert!(response.is_ok(), "response is not json")
        },
    }
}