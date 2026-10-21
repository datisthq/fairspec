from pydantic import BaseModel, Field


# Items stay `object`: fireargs validates every output, and validating each error
# or row of a large result through its full model costs seconds and a copy
class ReportResult(BaseModel):
    valid: bool = Field(description="Whether the validation passed without errors")
    errors: list[object] = Field(description="Array of validation errors encountered")


class TextResult(BaseModel):
    result: str


class Empty(BaseModel):
    pass


Records = list[object]
