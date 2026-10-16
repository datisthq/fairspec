from pydantic import BaseModel, ConfigDict


class FairspecModel(BaseModel):
    model_config = ConfigDict(revalidate_instances="never")
