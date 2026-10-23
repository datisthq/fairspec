use fairspec_library::fairspec_metadata::{Error, Result};
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
    ListToolsResult, PaginatedRequestParams, ServerCapabilities, ServerConfig,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt};
use serde_json::Value;

use crate::helpers::schema::TOOL_DEFINITIONS;
use crate::helpers::tool::{create_error_result, guard_tool, parse_tool_input};
use crate::tools::data::validate::validate_data_tool;
use crate::tools::data_schema::infer::infer_data_schema_tool;
use crate::tools::data_schema::validate::validate_data_schema_tool;
use crate::tools::dataset::infer::infer_dataset_tool;
use crate::tools::dataset::query::query_dataset_tool;
use crate::tools::dataset::validate::validate_dataset_tool;
use crate::tools::file::preview::preview_file_tool;
use crate::tools::file_dialect::infer::infer_file_dialect_tool;
use crate::tools::table::query::query_table_tool;
use crate::tools::table::validate::validate_table_tool;
use crate::tools::table_schema::infer::infer_table_schema_tool;
use crate::tools::table_schema::validate::validate_table_schema_tool;

/// The MCP server: every tool under the name the TypeScript server puts on the wire.
#[derive(Debug, Clone, Copy, Default)]
pub struct FairspecServer;

/// Serves the tools over stdin and stdout until the client disconnects. Blocks on its own
/// runtime; nothing but the transport writes to stdout.
pub fn serve_stdio() -> Result<()> {
    // fix-ssrf: set the library's HTTP host policy here, before any tool can run
    let to_error = |error: std::io::Error| Error::Load(error.to_string());
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(to_error)?;
    runtime.block_on(async {
        let service = FairspecServer
            .serve(rmcp::transport::stdio())
            .await
            .map_err(|error| Error::Load(error.to_string()))?;
        service
            .waiting()
            .await
            .map_err(|error| Error::Load(error.to_string()))?;
        Ok(())
    })
}

/// Runs a tool by its wire name on the calling thread.
pub fn call_tool(name: &str, arguments: Option<Value>) -> CallToolResult {
    macro_rules! run {
        ($tool:expr) => {
            guard_tool(name, parse_tool_input(arguments).and_then($tool))
        };
    }
    match name {
        "inferDataSchemaTool" => run!(infer_data_schema_tool),
        "inferDatasetTool" => run!(infer_dataset_tool),
        "inferFileDialectTool" => run!(infer_file_dialect_tool),
        "inferTableSchemaTool" => run!(infer_table_schema_tool),
        "previewFileTool" => run!(preview_file_tool),
        "queryDatasetTool" => run!(query_dataset_tool),
        "queryTableTool" => run!(query_table_tool),
        "validateDataSchemaTool" => run!(validate_data_schema_tool),
        "validateDatasetTool" => run!(validate_dataset_tool),
        "validateDataTool" => run!(validate_data_tool),
        "validateTableSchemaTool" => run!(validate_table_schema_tool),
        "validateTableTool" => run!(validate_table_tool),
        _ => CallToolResult::error(vec![ContentBlock::text(format!("Unknown tool: {name}"))]),
    }
}

impl ServerHandler for FairspecServer {
    fn get_info(&self) -> ServerConfig {
        let mut server_info = Implementation::default();
        server_info.name = "fairspec".to_string();
        server_info.version = env!("CARGO_PKG_VERSION").to_string();
        let mut capabilities = ServerCapabilities::builder().enable_tools().build();
        if let Some(tools) = capabilities.tools.as_mut() {
            tools.list_changed = Some(false);
        }
        ServerConfig::new(capabilities).with_server_info(server_info)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(TOOL_DEFINITIONS.clone()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<CallToolResponse, ErrorData> {
        let name = request.name.to_string();
        let arguments = request.arguments.map(Value::Object);
        // The library is sync and may block on files or the network
        let result = tokio::task::spawn_blocking(move || call_tool(&name, arguments))
            .await
            .unwrap_or_else(|error| create_error_result(&request.name, &error.to_string()));
        Ok(result.into())
    }
}
