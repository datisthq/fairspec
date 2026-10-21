from pydantic import BaseModel, ConfigDict
from pydantic.alias_generators import to_camel


class CommandInput(BaseModel):
    """The base of every command's input: `--json` and `--llms` use the TypeScript
    twin's camelCase keys while the flags stay kebab-case."""

    model_config = ConfigDict(alias_generator=to_camel, validate_by_name=True)
