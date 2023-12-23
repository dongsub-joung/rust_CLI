
pub mod service{
    use reqwest::{Client, Error};
    use serde_json::Error as OtherError;

    use crate::init::todo::Todo;

    pub fn handle_json(json: String) -> Todo{
        
    }
    pub fn get_todos(){
        
    }

    fn new_client()->Result<Client, reqwest::Error>{
        let client = Client::new();
        Ok(client)
    }
    pub async fn add_todos(body_txt: String) -> Result<String, Error> {
        let client= new_client().unwrap();
        // The URL you want to send the POST request to
        let url = "https://todo.ngrok.app/todos";
    
        // Create a JSON payload
        let value= format!("{}", body_txt.trim());
        let json_payload = serde_json::json!({
            "body_text": value,
        });
    
        // Send the POST request with the JSON payload
        let response = client
            .post(url)
            .json(&json_payload)
            .send()
            .await?;
    
        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            let body = response.text().await?;
            println!("Response body: {}", body);
        } else {
            println!("Request failed with status code: {}", response.status());
        }
        
        let CODE= String::from("SUCESSS");
        Ok(CODE)
    }
}