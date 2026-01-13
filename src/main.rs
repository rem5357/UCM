use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::stdio,
    ServiceExt,
};
use serde::Deserialize;

mod parser;
mod tools;
mod types;

use tools::{ucm_add, ucm_convert, ucm_diff, ucm_info, ucm_now, ucm_parse};

// Parameter types for tools
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ParseParams {
    #[schemars(description = "Natural language date expression")]
    pub expression: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DiffParams {
    #[schemars(description = "Start date (natural language or ISO format)")]
    pub from: String,
    #[schemars(description = "End date (natural language or ISO format)")]
    pub to: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddParams {
    #[schemars(description = "Base date (natural language or ISO format)")]
    pub date: String,
    #[schemars(description = "Duration to add (e.g., '3 weeks', '2 months', '-5 days')")]
    pub add: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ConvertParams {
    #[schemars(description = "Duration value to convert")]
    pub value: f64,
    #[schemars(description = "Source unit: days, weeks, months, years, hours, minutes, seconds")]
    pub from_unit: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InfoParams {
    #[schemars(description = "Date to analyze (natural language or ISO format)")]
    pub date: String,
}

#[derive(Clone)]
pub struct UcmServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl UcmServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get current date and time from system")]
    fn ucm_now(&self) -> String {
        let response = ucm_now();
        serde_json::to_string_pretty(&response).unwrap()
    }

    #[tool(description = "Parse natural language date expression (e.g., 'next wednesday', 'yesterday', 'october 22')")]
    fn ucm_parse(&self, Parameters(params): Parameters<ParseParams>) -> String {
        match ucm_parse(&params.expression) {
            Ok(response) => serde_json::to_string_pretty(&response).unwrap(),
            Err(error) => serde_json::to_string_pretty(&error).unwrap(),
        }
    }

    #[tool(description = "Calculate difference between two dates in multiple units")]
    fn ucm_diff(&self, Parameters(params): Parameters<DiffParams>) -> String {
        match ucm_diff(&params.from, &params.to) {
            Ok(response) => serde_json::to_string_pretty(&response).unwrap(),
            Err(error) => serde_json::to_string_pretty(&error).unwrap(),
        }
    }

    #[tool(description = "Add duration to a date (e.g., '3 weeks', '-5 days', '2 months')")]
    fn ucm_add(&self, Parameters(params): Parameters<AddParams>) -> String {
        match ucm_add(&params.date, &params.add) {
            Ok(response) => serde_json::to_string_pretty(&response).unwrap(),
            Err(error) => serde_json::to_string_pretty(&error).unwrap(),
        }
    }

    #[tool(description = "Convert duration between units (e.g., 3823 days to years/months/weeks)")]
    fn ucm_convert(&self, Parameters(params): Parameters<ConvertParams>) -> String {
        match ucm_convert(params.value, &params.from_unit) {
            Ok(response) => serde_json::to_string_pretty(&response).unwrap(),
            Err(error) => serde_json::to_string_pretty(&error).unwrap(),
        }
    }

    #[tool(description = "Get detailed information about a date")]
    fn ucm_info(&self, Parameters(params): Parameters<InfoParams>) -> String {
        match ucm_info(&params.date) {
            Ok(response) => serde_json::to_string_pretty(&response).unwrap(),
            Err(error) => serde_json::to_string_pretty(&error).unwrap(),
        }
    }
}

#[tool_handler]
impl rmcp::ServerHandler for UcmServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Universal Calendar Manager - Date/time calculations for Claude Desktop. \
                 Use ucm_now for current time, ucm_parse for natural language dates, \
                 ucm_diff for date differences, ucm_add for date arithmetic, \
                 ucm_convert for duration conversions, ucm_info for date details."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = UcmServer::new().serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
