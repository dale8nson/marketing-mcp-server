use crate::graphql::{self, Query};
use async_graphql::{Schema, *};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        tool::{ToolCallContext, ToolRouter},
        wrapper::Parameters,
    },
    model::*,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{Level, debug, event, info, instrument};

#[derive(Clone, Debug)]
pub struct Mcp {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl Mcp {
    #[instrument]
    #[tool(name = "query")]
    pub fn query(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }

    pub fn new() -> Self {
        println!("Mcp::new()");
        tracing::error!("test");
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for Mcp {
    fn get_info(&self) -> ServerInfo {
        tracing::info!("Mcp::get_info()");
        ServerInfo {
            instructions: Some("A simple MCP".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// #[tool_handler]
// impl ServerHandler for Mcp {}
