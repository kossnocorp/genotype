from typing import Optional
from genotype import Model


class User(Model):
    first_name: str
    last_name: Optional[str]


def test_parse():
    user = User.model_validate_json('{"firstName": "Sasha", "lastName": "Koss"}')
    assert user.first_name == "Sasha"
    assert user.last_name == "Koss"


def test_serialize():
    user = User(first_name="Sasha", last_name="Koss")
    assert user.model_dump_json() == '{"firstName":"Sasha","lastName":"Koss"}'
