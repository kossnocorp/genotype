from typing import Any
from pydantic import BaseModel, ConfigDict


class GenotypeModel(BaseModel):
    model_config = ConfigDict(populate_by_name=True)

    def __init__(self, **data: Any) -> None:
        super().__init__(**data)

    def model_dump(self, **kwargs: Any) -> dict[str, Any]:
        kwargs.setdefault("by_alias", True)
        return super().model_dump(**kwargs)

    def model_dump_json(self, **kwargs: Any) -> str:
        kwargs.setdefault("by_alias", True)
        return super().model_dump_json(**kwargs)


__all__ = ["GenotypeModel"]
