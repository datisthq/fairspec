from importlib.metadata import version

from mcp.server.mcpserver import MCPServer

from .tools.data.validate import validate_data_tool
from .tools.data_schema.infer import infer_data_schema_tool
from .tools.data_schema.validate import validate_data_schema_tool
from .tools.dataset.infer import infer_dataset_tool
from .tools.dataset.validate import validate_dataset_tool
from .tools.file_dialect.infer import infer_file_dialect_tool
from .tools.table.query import query_table_tool
from .tools.table.validate import validate_table_tool
from .tools.table_schema.infer import infer_table_schema_tool
from .tools.table_schema.validate import validate_table_schema_tool

server = MCPServer(name="fairspec", version=version("fairspec-mcp-server"))

server.add_tool(infer_data_schema_tool, name="inferDataSchemaTool")
server.add_tool(infer_dataset_tool, name="inferDatasetTool")
server.add_tool(infer_file_dialect_tool, name="inferFileDialectTool")
server.add_tool(infer_table_schema_tool, name="inferTableSchemaTool")
server.add_tool(query_table_tool, name="queryTableTool")
server.add_tool(validate_data_schema_tool, name="validateDataSchemaTool")
server.add_tool(validate_dataset_tool, name="validateDatasetTool")
server.add_tool(validate_data_tool, name="validateDataTool")
server.add_tool(validate_table_schema_tool, name="validateTableSchemaTool")
server.add_tool(validate_table_tool, name="validateTableTool")
