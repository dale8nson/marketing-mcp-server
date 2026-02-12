use std::sync::{Arc, RwLock};

use crate::graphql::OAuth;

#[derive(Clone, Debug)]
pub struct AppState {
    pub client: reqwest::Client,
    pub salesforce_initial_access_token: Arc<String>,
    pub salesforce_consumer_key: Arc<String>,
    pub salesforce_consumer_secret: Arc<String>,
    pub salesforce_login_url: Arc<String>,
    pub salesforce_graphql_endpoint: Arc<String>,
    pub salesforce_access_token: fn() -> String,
}

impl Default for AppState {
    fn default() -> Self {
        let client = reqwest::Client::default();

        let salesforce_initial_access_token =
            Arc::new(std::env::var("SALESFORCE_INITIAL_ACCESS_TOKEN").unwrap_or_default());
        let salesforce_consumer_key =
            Arc::new(std::env::var("SALESFORCE_CONSUMER_KEY").unwrap_or_default());
        let salesforce_consumer_secret =
            Arc::new(std::env::var("SALESFORCE_CONSUMER_SECRET").unwrap_or_default());
        let salesforce_login_url =
            Arc::new(std::env::var("SALESFORCE_LOGIN_URL").unwrap_or_default());
        let salesforce_graphql_endpoint =
            Arc::new(std::env::var("SALESFORCE_GRAPHQL_ENDPOINT").unwrap_or_default());

        let salesforce_access_token = &mut async move || {
            let mut token_request_url = String::from(&*salesforce_login_url.clone());
            token_request_url.push_str("/services/oauth2/token");
            let token_req = client
                .post(token_request_url)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(format!(
                    "grant_type=client_credentials\
              &client_id={}\
              &client_secret={}",
                    salesforce_consumer_key.clone(),
                    salesforce_consumer_secret.clone()
                ))
                .build()
                .unwrap();
            let res = client.execute(token_req).await;
            println!("{res:#?}");
            let json = res.unwrap().json::<OAuth>().await.unwrap();
            println!("{json:#?}");

            json.access_token
        };

        Self {
            client,
            salesforce_initial_access_token,
            salesforce_consumer_key,
            salesforce_consumer_secret,
            salesforce_login_url,
            salesforce_graphql_endpoint,
            salesforce_access_token,
        }
    }
}
