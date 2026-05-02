from typing import Literal
from typing import Optional
from genotype import Model

type StatusCode = Literal[200] | Literal[500]

class Status(Model):
    status: Literal["success"] | Literal["failure"]
    code: StatusCode


def test_model_parse():
    status = Status.model_validate_json('{"status": "success", "code": 200}')
    assert status.status == "success"
    assert status.code == 200
