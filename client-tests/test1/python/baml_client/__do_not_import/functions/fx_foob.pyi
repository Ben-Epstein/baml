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

from ..types.classes.cls_inputtype import InputType
from ..types.classes.cls_inputtype2 import InputType2
from ..types.classes.cls_outputtype import OutputType
from ..types.enums.enm_sentiment import Sentiment
from typing import Protocol, runtime_checkable


import typing

import pytest

ImplName = typing.Literal["FooImpl"]

T = typing.TypeVar("T", bound=typing.Callable[..., typing.Any])
CLS = typing.TypeVar("CLS", bound=type)


IFooBOutput = OutputType

@runtime_checkable
class IFooB(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: InputType

    Returns:
        OutputType
    """

    async def __call__(self, arg: InputType, /) -> OutputType:
        ...


class BAMLFooBImpl:
    async def run(self, arg: InputType, /) -> OutputType:
        ...

class IBAMLFooB:
    def register_impl(
        self, name: ImplName
    ) -> typing.Callable[[IFooB], IFooB]:
        ...

    def get_impl(self, name: ImplName) -> BAMLFooBImpl:
        ...

    @typing.overload
    def test(self, test_function: T) -> T:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the FooBInterface.

        Args:
            test_function : T
                The test function to be decorated.

        Usage:
            ```python
            # All implementations will be tested.

            @baml.FooB.test
            def test_logic(FooBImpl: IFooB) -> None:
                result = await FooBImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, *, exclude_impl: typing.Iterable[ImplName]) -> pytest.MarkDecorator:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the FooBInterface.

        Args:
            exclude_impl : Iterable[ImplName]
                The names of the implementations to exclude from testing.

        Usage:
            ```python
            # All implementations except "FooImpl" will be tested.

            @baml.FooB.test(exclude_impl=["FooImpl"])
            def test_logic(FooBImpl: IFooB) -> None:
                result = await FooBImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, test_class: typing.Type[CLS]) -> typing.Type[CLS]:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the FooBInterface.

        Args:
            test_class : Type[CLS]
                The test class to be decorated.

        Usage:
        ```python
        # All implementations will be tested in every test method.

        @baml.FooB.test
        class TestClass:
            def test_a(self, FooBImpl: IFooB) -> None:
                ...
            def test_b(self, FooBImpl: IFooB) -> None:
                ...
        ```
        """
        ...

BAMLFooB: IBAMLFooB
