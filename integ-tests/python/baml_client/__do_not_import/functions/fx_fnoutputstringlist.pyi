# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from baml_core.stream import AsyncStream
from typing import Callable, List, Protocol, runtime_checkable


import typing

import pytest
from contextlib import contextmanager
from unittest import mock

ImplName = typing.Literal["v1"]

T = typing.TypeVar("T", bound=typing.Callable[..., typing.Any])
CLS = typing.TypeVar("CLS", bound=type)


IFnOutputStringListOutput = List[str]

@runtime_checkable
class IFnOutputStringList(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: str

    Returns:
        List[str]
    """

    async def __call__(self, arg: str, /) -> List[str]:
        ...

   

@runtime_checkable
class IFnOutputStringListStream(Protocol):
    """
    This is the interface for a stream function.

    Args:
        arg: str

    Returns:
        AsyncStream[List[str], List[str]]
    """

    def __call__(self, arg: str, /) -> AsyncStream[List[str], List[str]]:
        ...
class BAMLFnOutputStringListImpl:
    async def run(self, arg: str, /) -> List[str]:
        ...
    
    def stream(self, arg: str, /) -> AsyncStream[List[str], List[str]]:
        ...

class IBAMLFnOutputStringList:
    def register_impl(
        self, name: ImplName
    ) -> typing.Callable[[IFnOutputStringList, IFnOutputStringListStream], None]:
        ...

    async def __call__(self, arg: str, /) -> List[str]:
        ...

    def stream(self, arg: str, /) -> AsyncStream[List[str], List[str]]:
        ...

    def get_impl(self, name: ImplName) -> BAMLFnOutputStringListImpl:
        ...

    @contextmanager
    def mock(self) -> typing.Generator[mock.AsyncMock, None, None]:
        """
        Utility for mocking the FnOutputStringListInterface.

        Usage:
            ```python
            # All implementations are mocked.

            async def test_logic() -> None:
                with baml.FnOutputStringList.mock() as mocked:
                    mocked.return_value = ...
                    result = await FnOutputStringListImpl(...)
                    assert mocked.called
            ```
        """
        ...

    @typing.overload
    def test(self, test_function: T) -> T:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the FnOutputStringListInterface.

        Args:
            test_function : T
                The test function to be decorated.

        Usage:
            ```python
            # All implementations will be tested.

            @baml.FnOutputStringList.test
            async def test_logic(FnOutputStringListImpl: IFnOutputStringList) -> None:
                result = await FnOutputStringListImpl(...)
            ```
        """
        ...

    @typing.overload
    def test(self, *, exclude_impl: typing.Iterable[ImplName] = [], stream: bool = False) -> pytest.MarkDecorator:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the FnOutputStringListInterface.

        Args:
            exclude_impl : Iterable[ImplName]
                The names of the implementations to exclude from testing.
            stream: bool
                If set, will return a streamable version of the test function.

        Usage:
            ```python
            # All implementations except the given impl will be tested.

            @baml.FnOutputStringList.test(exclude_impl=["implname"])
            async def test_logic(FnOutputStringListImpl: IFnOutputStringList) -> None:
                result = await FnOutputStringListImpl(...)
            ```

            ```python
            # Streamable version of the test function.

            @baml.FnOutputStringList.test(stream=True)
            async def test_logic(FnOutputStringListImpl: IFnOutputStringListStream) -> None:
                async for result in FnOutputStringListImpl(...):
                    ...
            ```
        """
        ...

    @typing.overload
    def test(self, test_class: typing.Type[CLS]) -> typing.Type[CLS]:
        """
        Provides a pytest.mark.parametrize decorator to facilitate testing different implementations of
        the FnOutputStringListInterface.

        Args:
            test_class : Type[CLS]
                The test class to be decorated.

        Usage:
        ```python
        # All implementations will be tested in every test method.

        @baml.FnOutputStringList.test
        class TestClass:
            def test_a(self, FnOutputStringListImpl: IFnOutputStringList) -> None:
                ...
            def test_b(self, FnOutputStringListImpl: IFnOutputStringList) -> None:
                ...
        ```
        """
        ...

BAMLFnOutputStringList: IBAMLFnOutputStringList