use crate::openaq::CountriesGetV3CountriesRequest;
use crate::openaq::CountriesGetV3CountriesRequestQuery;
use crate::openaq::CountriesGetV3CountriesResponse;
use crate::openaq::CountryGetV3CountriesCountriesIdRequest;
use crate::openaq::CountryGetV3CountriesCountriesIdRequestPath;
use crate::openaq::LocationLatestGetV3LocationsLocationsIdLatestRequest;
use crate::openaq::LocationLatestGetV3LocationsLocationsIdLatestRequestPath;
use crate::openaq::LocationLatestGetV3LocationsLocationsIdLatestRequestQuery;
use crate::openaq::LocationLatestGetV3LocationsLocationsIdLatestResponse;
use crate::openaq::LocationsGetV3LocationsRequest;
use crate::openaq::LocationsGetV3LocationsRequestQuery;
use crate::openaq::LocationsGetV3LocationsResponse;
use crate::openaq::OpenAQClient;
use reqwest::Client;
use reqwest::header;
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct NatureIq {
    tool_router: ToolRouter<Self>,
}

impl Default for NatureIq {
    fn default() -> Self {
        Self::new()
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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CountriesListRequest {
    #[schemars(
        description = "Paginate through results. e.g. page=1 will return first page of results"
    )]
    pub page: Option<i64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CountryDetailsRequest {
    #[schemars(description = "country_id uniquely identifies Countries in the OpenAQ data set")]
    pub countries_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LocationsListRequest {
    #[schemars(
        description = "Paginate through results. e.g. page=1 will return first page of results"
    )]
    pub page: Option<i64>,
    #[schemars(description = "country_id uniquely identifies Countries in the OpenAQ data set")]
    pub countries_id: Option<Value>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LocationLatestRequest {
    #[schemars(description = "locations_id uniquely identifies Locations in the OpenAQ data set")]
    pub locations_id: i64,
    #[schemars(
        description = "Paginate through results. e.g. page=1 will return first page of results"
    )]
    pub page: Option<i64>,
}

#[tool_router]
impl NatureIq {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Lists Countries in the Open Air Quality data set")]
    async fn countries_in_open_aq(
        &self,
        Parameters(CountriesListRequest { page }): Parameters<CountriesListRequest>,
    ) -> String {
        let query = CountriesGetV3CountriesRequestQuery {
            order_by: None,
            sort_order: None,
            providers_id: None,
            parameters_id: None,
            limit: None,
            page,
        };
        let request = CountriesGetV3CountriesRequest { query };
        let client = prepare_client();
        let raw_response = client.countries_get_v3_countries(request).await.unwrap();
        format_countries_response(raw_response)
    }

    #[tool(description = "Details Air Quality of a Country in the Open Air Quality data set")]
    async fn country_in_open_aq(
        &self,
        Parameters(CountryDetailsRequest { countries_id }): Parameters<CountryDetailsRequest>,
    ) -> String {
        let path = CountryGetV3CountriesCountriesIdRequestPath { countries_id };
        let request = CountryGetV3CountriesCountriesIdRequest { path };
        let client = prepare_client();
        let raw_response = client
            .country_get_v3_countries_countries_id(request)
            .await
            .unwrap();
        format_countries_response(raw_response)
    }

    #[tool(description = "Lists Locations in the Open Air Quality data set")]
    async fn locations_in_open_aq(
        &self,
        Parameters(LocationsListRequest { page, countries_id }): Parameters<LocationsListRequest>,
    ) -> String {
        let query = LocationsGetV3LocationsRequestQuery {
            coordinates: None,
            radius: None,
            providers_id: None,
            parameters_id: None,
            limit: None,
            page,
            owner_contacts_id: None,
            manufacturers_id: None,
            order_by: None,
            sort_order: None,
            licenses_id: None,
            monitor: None,
            mobile: None,
            instruments_id: None,
            iso: None,
            countries_id,
            bbox: None,
        };
        let request = LocationsGetV3LocationsRequest { query };
        let client = prepare_client();
        let raw_response = client.locations_get_v3_locations(request).await.unwrap();
        format_locations_response(raw_response)
    }

    #[tool(description = "Latest Open Air Quality Measurements for a Location")]
    async fn latest_measurements_for_location(
        &self,
        Parameters(LocationLatestRequest { locations_id, page }): Parameters<LocationLatestRequest>,
    ) -> String {
        let path = LocationLatestGetV3LocationsLocationsIdLatestRequestPath { locations_id };
        let query = LocationLatestGetV3LocationsLocationsIdLatestRequestQuery {
            limit: Some(100),
            page,
            datetime_min: None,
        };
        let request = LocationLatestGetV3LocationsLocationsIdLatestRequest { path, query };
        let client = prepare_client();
        let raw_response = client
            .location_latest_get_v3_locations_locations_id_latest(request)
            .await
            .unwrap();
        format_locations_latest_response(raw_response)
    }
}

fn prepare_client() -> OpenAQClient {
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
    OpenAQClient::with_client("https://api.openaq.org/", client)
        .expect("OpenAQClient should create without error")
}

fn format_countries_response(raw_response: CountriesGetV3CountriesResponse) -> String {
    if let CountriesGetV3CountriesResponse::Ok(response) = raw_response {
        format!("{0:?}", response.results)
    } else {
        println!("an error {:?}", raw_response);
        "Request failed".to_string()
    }
}

fn format_locations_response(raw_response: LocationsGetV3LocationsResponse) -> String {
    if let LocationsGetV3LocationsResponse::Ok(response) = raw_response {
        format!("{0:?}", response.results)
    } else {
        println!("an error {:?}", raw_response);
        "Request failed".to_string()
    }
}

fn format_locations_latest_response(
    raw_response: LocationLatestGetV3LocationsLocationsIdLatestResponse,
) -> String {
    if let LocationLatestGetV3LocationsLocationsIdLatestResponse::Ok(response) = raw_response {
        format!("{0:?}", response.results)
    } else {
        println!("an error {:?}", raw_response);
        "Request failed".to_string()
    }
}
