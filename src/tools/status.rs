use crate::types::StatusResponse;

/// Current build number - increment with each build
pub const BUILD_NUMBER: u32 = 2;

/// Get UCM server status and version information
pub fn ucm_status() -> StatusResponse {
    StatusResponse {
        name: "UCM - Universal Calendar Manager".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        build: BUILD_NUMBER,
        description: "MCP server providing date/time calculations for Claude Desktop. \
            Addresses LLM limitations with temporal reasoning and date arithmetic.".to_string(),
        tools_available: vec![
            "ucm_now".to_string(),
            "ucm_parse".to_string(),
            "ucm_diff".to_string(),
            "ucm_add".to_string(),
            "ucm_convert".to_string(),
            "ucm_info".to_string(),
            "ucm_status".to_string(),
            "ucm_instructions".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_returns_valid_response() {
        let response = ucm_status();
        assert_eq!(response.name, "UCM - Universal Calendar Manager");
        assert!(!response.version.is_empty());
        assert!(response.build >= 1);
        assert!(!response.tools_available.is_empty());
    }
}
