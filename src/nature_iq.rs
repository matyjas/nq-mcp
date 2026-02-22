use crate::openaq::CountriesGetV3CountriesRequest;
use crate::openaq::CountriesGetV3CountriesRequestQuery;
use crate::openaq::CountriesGetV3CountriesResponse;
use crate::openaq::OpenAQClient;

use reqwest::header;
use reqwest::Client;
use rmcp::{
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ServerHandler,
};

#[derive(Debug, Clone)]
pub struct NatureIq {
    tool_router: ToolRouter<Self>,
}

impl Default for NatureIq {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl NatureIq {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Lists Countries in the Open Air Quality data set")]
    async fn countries_in_open_aq(&self) -> String {
        let query = CountriesGetV3CountriesRequestQuery {
            order_by: None,
            sort_order: None,
            providers_id: None,
            parameters_id: None,
            limit: None,
            page: None,
        };
        let request = CountriesGetV3CountriesRequest { query };
        let open_aq_api_key =
            std::env::var("OPEN_AQ_API_KEY").expect("OPEN_AQ_API_KEY env var should be set");
        let header_value = header::HeaderValue::from_str(&open_aq_api_key)
            .expect("OPEN_AQ_API_KEY should have reasonable value for header");
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-Key", header_value);
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("");
        let client = OpenAQClient::with_client("https://api.openaq.org/", client)
            .expect("OpenAQClient should create without error");
        let raw_response = client.countries_get_v3_countries(request).await.unwrap();
        if let CountriesGetV3CountriesResponse::Ok(response) = raw_response {
            format!("{0:?}", response.results)
        } else {
            println!("an error {:?}", raw_response);
            "Request failed".to_string()
        }
    }
}

#[tool_handler]
impl ServerHandler for NatureIq {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Source of nature intelligence".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
