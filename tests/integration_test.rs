use reqwest::Client;
use serde_json::{Value, json};

#[tokio::test]
async fn test_api() {
    let base = "http://localhost:8000";
    let client = Client::new();

    let res = client.get(base).send().await.unwrap();
    assert!(res.status().is_success());

    let payload = json!({
        "title": "Lorem ipsum",
        "value": 100,
        "code": "LIPS01",
        "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
    });
    let res = client
        .post(format!("{}/exampleitems/", base))
        .json(&payload)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let doc: Value = res.json().await.unwrap();
    let id = doc["id"].as_str().unwrap().to_string();
    assert_eq!(doc["title"], "Lorem ipsum");
    assert_eq!(doc["value"], 100);
    assert_eq!(doc["code"], "LIPS01");

    let res = client
        .get(format!("{}/exampleitems/", base))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let items: Vec<Value> = res.json().await.unwrap();
    assert!(items.iter().any(|i| i["id"] == id));

    let res = client
        .get(format!("{}/exampleitems/{}", base, id))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    let res = client
        .delete(format!("{}/exampleitems/{}", base, id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 204);
}
