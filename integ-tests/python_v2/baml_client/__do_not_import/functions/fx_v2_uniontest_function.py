# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..types.classes.cls_uniontest_returntypev2 import UnionTest_ReturnTypev2
from ..types.enums.enm_datatype import DataType
from ..types.partial.classes.cls_uniontest_returntypev2 import PartialUnionTest_ReturnTypev2
from baml_core.stream import AsyncStream
from baml_lib._impl.functions import BaseBAMLFunction
from typing import AsyncIterator, Callable, Protocol, Union, runtime_checkable


IV2_UnionTest_FunctionOutput = Union[UnionTest_ReturnTypev2, DataType]

@runtime_checkable
class IV2_UnionTest_Function(Protocol):
    """
    This is the interface for a function.

    Args:
        input: Union[str, bool]

    Returns:
        Union[UnionTest_ReturnTypev2, DataType]
    """

    async def __call__(self, *, input: Union[str, bool]) -> Union[UnionTest_ReturnTypev2, DataType]:
        ...

   

@runtime_checkable
class IV2_UnionTest_FunctionStream(Protocol):
    """
    This is the interface for a stream function.

    Args:
        input: Union[str, bool]

    Returns:
        AsyncStream[Union[UnionTest_ReturnTypev2, DataType], Union[UnionTest_ReturnTypev2, DataType]]
    """

    def __call__(self, *, input: Union[str, bool]
) -> AsyncStream[Union[UnionTest_ReturnTypev2, DataType], Union[UnionTest_ReturnTypev2, DataType]]:
        ...
class IBAMLV2_UnionTest_Function(BaseBAMLFunction[Union[UnionTest_ReturnTypev2, DataType], Union[UnionTest_ReturnTypev2, DataType]]):
    def __init__(self) -> None:
        super().__init__(
            "V2_UnionTest_Function",
            IV2_UnionTest_Function,
            ["default_config"],
        )

    async def __call__(self, *args, **kwargs) -> Union[UnionTest_ReturnTypev2, DataType]:
        return await self.get_impl("default_config").run(*args, **kwargs)
    
    def stream(self, *args, **kwargs) -> AsyncStream[Union[UnionTest_ReturnTypev2, DataType], Union[UnionTest_ReturnTypev2, DataType]]:
        res = self.get_impl("default_config").stream(*args, **kwargs)
        return res

BAMLV2_UnionTest_Function = IBAMLV2_UnionTest_Function()

__all__ = [ "BAMLV2_UnionTest_Function" ]