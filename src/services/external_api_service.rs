use reqwest::Client;

pub async fn call_external_api() -> Result<String, reqwest::Error> {
    // Create an HTTP client
    let client = Client::new();

    // Make a GET request to the external API
    let response = client.get("http://localhost:8080/customers")
        .send()
        .await;

    // Log details about the response or error
    match response {
        Ok(resp) => {
            let text = resp.text().await?;
            Ok(text)
        },
        Err(err) => {
            println!("Erro ao fazer requisição: {:?}", err);
            Err(err)
        }
    }
}
