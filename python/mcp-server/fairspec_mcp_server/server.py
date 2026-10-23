import os
from importlib.metadata import version

from fairspec_library import set_host_policy
from mcp.server.mcpserver import MCPServer

from .helpers.tool import guard_tool
from .tools.data.validate import validate_data_tool
from .tools.data_schema.infer import infer_data_schema_tool
from .tools.data_schema.validate import validate_data_schema_tool
from .tools.dataset.infer import infer_dataset_tool
from .tools.dataset.query import query_dataset_tool
from .tools.dataset.validate import validate_dataset_tool
from .tools.file.preview import preview_file_tool
from .tools.file_dialect.infer import infer_file_dialect_tool
from .tools.table.query import query_table_tool
from .tools.table.validate import validate_table_tool
from .tools.table_schema.infer import infer_table_schema_tool
from .tools.table_schema.validate import validate_table_schema_tool

server = MCPServer(name="fairspec", version=version("fairspec-mcp-server"))

server.add_tool(guard_tool(infer_data_schema_tool), name="inferDataSchemaTool")
server.add_tool(guard_tool(infer_dataset_tool), name="inferDatasetTool")
server.add_tool(guard_tool(infer_file_dialect_tool), name="inferFileDialectTool")
server.add_tool(guard_tool(infer_table_schema_tool), name="inferTableSchemaTool")
server.add_tool(guard_tool(preview_file_tool), name="previewFileTool")
server.add_tool(guard_tool(query_dataset_tool), name="queryDatasetTool")
server.add_tool(guard_tool(query_table_tool), name="queryTableTool")
server.add_tool(guard_tool(validate_data_schema_tool), name="validateDataSchemaTool")
server.add_tool(guard_tool(validate_dataset_tool), name="validateDatasetTool")
server.add_tool(guard_tool(validate_data_tool), name="validateDataTool")
server.add_tool(guard_tool(validate_table_schema_tool), name="validateTableSchemaTool")
server.add_tool(guard_tool(validate_table_tool), name="validateTableTool")


def start_server() -> None:
    """Serve over stdio; remote fetches reach only public hosts unless FAIRSPEC_HOST_POLICY is set."""
    if not os.environ.get("FAIRSPEC_HOST_POLICY"):
        set_host_policy("public")
    server.run("stdio")
