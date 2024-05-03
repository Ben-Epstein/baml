# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from baml_core.stream import AsyncStream
from baml_lib._impl.functions import BaseBAMLFunction
from typing import AsyncIterator, Callable, List, Protocol, runtime_checkable


IV2_FnOutputStringListOutput = List[str]

@runtime_checkable
class IV2_FnOutputStringList(Protocol):
    """
    This is the interface for a function.

    Args:
        input: str

    Returns:
        List[str]
    """

    async def __call__(self, *, input: str) -> List[str]:
        ...

   

@runtime_checkable
class IV2_FnOutputStringListStream(Protocol):
    """
    This is the interface for a stream function.

    Args:
        input: str

    Returns:
        AsyncStream[List[str], List[str]]
    """

    def __call__(self, *, input: str
) -> AsyncStream[List[str], List[str]]:
        ...
class IBAMLV2_FnOutputStringList(BaseBAMLFunction[List[str], List[str]]):
    def __init__(self) -> None:
        super().__init__(
            "V2_FnOutputStringList",
            IV2_FnOutputStringList,
            ["default_config"],
        )

    async def __call__(self, *args, **kwargs) -> List[str]:
        return await self.get_impl("default_config").run(*args, **kwargs)
    
    def stream(self, *args, **kwargs) -> AsyncStream[List[str], List[str]]:
        res = self.get_impl("default_config").stream(*args, **kwargs)
        return res

BAMLV2_FnOutputStringList = IBAMLV2_FnOutputStringList()

__all__ = [ "BAMLV2_FnOutputStringList" ]