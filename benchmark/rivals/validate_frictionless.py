from common import Schema, load_schema, print_result, read_arguments
from frictionless import Resource, Schema as TableSchema, validate

TYPES = {
    "integer": "integer",
    "number": "number",
    "string": "string",
    "boolean": "boolean",
    "date-time": "datetime",
}


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = convert_schema(load_schema(schema_path))
    resource = Resource(path=csv_path, schema=schema)
    report = validate(resource, limit_errors=10**9)
    print_result(sum(len(task.errors) for task in report.tasks))


def convert_schema(schema: Schema) -> TableSchema:
    fields = []
    for column in schema.columns:
        constraints: dict[str, object] = {"required": not column.nullable}
        if column.minimum is not None:
            constraints["minimum"] = column.minimum
        if column.maximum is not None:
            constraints["maximum"] = column.maximum
        if column.min_length is not None:
            constraints["minLength"] = column.min_length
        if column.pattern is not None:
            constraints["pattern"] = column.pattern.removeprefix("^").removesuffix("$")
        if column.enum is not None:
            constraints["enum"] = column.enum
        field: dict[str, object] = {
            "name": column.name,
            "type": TYPES[column.kind],
            "constraints": constraints,
        }
        if column.kind == "date-time":
            field["format"] = "%Y-%m-%dT%H:%M:%S"
        fields.append(field)
    return TableSchema.from_descriptor(
        {"fields": fields, "primaryKey": schema.primary_key}
    )


if __name__ == "__main__":
    main()
