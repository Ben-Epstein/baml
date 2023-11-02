# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.0.1
# Generated Date: __DATE__
# Generated by: vbv

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long

from ..types.classes.cls_type1 import Type1
from ..types.classes.cls_type2 import Type2
from baml_core._impl.functions import BaseBAMLFunction
from typing import Protocol, runtime_checkable


IFunctionOneOutput = Type2

@runtime_checkable
class IFunctionOne(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: Type1

    Returns:
        Type2
    """

    async def __call__(self, arg: Type1, /) -> Type2:
        ...


class IBAMLFunctionOne(BaseBAMLFunction[Type2]):
    def __init__(self) -> None:
        super().__init__(
            "FunctionOne",
            IFunctionOne,
            [],
        )

BAMLFunctionOne = IBAMLFunctionOne()

__all__ = [ "BAMLFunctionOne" ]
