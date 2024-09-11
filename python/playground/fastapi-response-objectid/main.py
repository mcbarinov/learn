from datetime import datetime
from typing import Any, Annotated

from bson import ObjectId
from fastapi import FastAPI
from pydantic import BaseModel, Field
from pydantic_core import core_schema


class ObjectIdAnnotation:
    @classmethod
    def __get_pydantic_core_schema__(
            cls, _source_type: Any, _handler: Any
    ) -> core_schema.CoreSchema:
        object_id_schema = core_schema.chain_schema(
            [
                core_schema.str_schema(),
                core_schema.no_info_plain_validator_function(cls.validate),
            ]
        )
        return core_schema.json_or_python_schema(
            json_schema=core_schema.str_schema(),
            python_schema=core_schema.union_schema(
                [core_schema.is_instance_schema(ObjectId), object_id_schema]
            ),
            serialization=core_schema.plain_serializer_function_ser_schema(
                lambda x: str(x)
            ),
        )

    @classmethod
    def validate(cls, value):
        if not ObjectId.is_valid(value):
            raise ValueError("Invalid id")

        return ObjectId(value)


PydanticObjectId = Annotated[ObjectId, ObjectIdAnnotation]


class Data(BaseModel):
    id: PydanticObjectId = Field(..., alias="_id")
    name: str
    tags: list[str]
    created_at: datetime = Field(default_factory=datetime.utcnow)

    # model_config = ConfigDict(arbitrary_types_allowed=True)


app = FastAPI()


@app.get("/")
def handler1() -> PydanticObjectId:
    return ObjectId()


@app.get("/test/{pk}")
def handler2(pk: PydanticObjectId) -> PydanticObjectId:
    print(type(pk))
    return pk


@app.get("/data")
def handler3() -> Data:
    return Data(_id=ObjectId(), name="test", tags=["a", "b"])
