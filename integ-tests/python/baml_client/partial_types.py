###############################################################################
#
#  Welcome to Baml! To use this generated code, please run the following:
#
#  $ pip install baml
#
###############################################################################

# This file was generated by BAML: please do not edit it. Instead, edit the
# BAML files and re-generate this code.
#
# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off
import baml_py
from enum import Enum
from pydantic import BaseModel, ConfigDict
from typing import Dict, Generic, List, Optional, TypeVar, Union, Literal

from . import types
from .types import Checked, Check

###############################################################################
#
#  These types are used for streaming, for when an instance of a type
#  is still being built up and any of its fields is not yet fully available.
#
###############################################################################

T = TypeVar('T')
class StreamState(BaseModel, Generic[T]):
    value: T
    completion_state: Literal["Pending", "Incomplete", "Complete"]


class BigNumbers(BaseModel):
    a: Optional[int] = None
    b: Optional[float] = None

class BinaryNode(BaseModel):
    data: Optional[int] = None
    left: Optional["BinaryNode"] = None
    right: Optional["BinaryNode"] = None

class Blah(BaseModel):
    prop4: Optional[str] = None

class BlockConstraint(BaseModel):
    foo: Optional[int] = None
    bar: Optional[str] = None

class BlockConstraintForParam(BaseModel):
    bcfp: Optional[int] = None
    bcfp2: Optional[str] = None

class BookOrder(BaseModel):
    orderId: Optional[str] = None
    title: Optional[str] = None
    quantity: Optional[int] = None
    price: Optional[float] = None

class ClassOptionalOutput(BaseModel):
    prop1: Optional[str] = None
    prop2: Optional[str] = None

class ClassOptionalOutput2(BaseModel):
    prop1: Optional[str] = None
    prop2: Optional[str] = None
    prop3: Optional["Blah"] = None

class ClassWithImage(BaseModel):
    myImage: Optional[baml_py.Image] = None
    param2: Optional[str] = None
    fake_image: Optional["FakeImage"] = None

class CompoundBigNumbers(BaseModel):
    big: Optional["BigNumbers"] = None
    big_nums: List["BigNumbers"]
    another: Optional["BigNumbers"] = None

class ContactInfo(BaseModel):
    primary: Optional[Union["PhoneNumber", "EmailAddress"]] = None
    secondary: Optional[Union["PhoneNumber", "EmailAddress", Optional[None]]] = None

class CustomTaskResult(BaseModel):
    bookOrder: Optional[Union["BookOrder", Optional[None]]] = None
    flightConfirmation: Optional[Union["FlightConfirmation", Optional[None]]] = None
    groceryReceipt: Optional[Union["GroceryReceipt", Optional[None]]] = None

class DummyOutput(BaseModel):
    model_config = ConfigDict(extra='allow')
    nonce: Optional[str] = None
    nonce2: Optional[str] = None

class DynInputOutput(BaseModel):
    model_config = ConfigDict(extra='allow')
    testKey: Optional[str] = None

class DynamicClassOne(BaseModel):
    model_config = ConfigDict(extra='allow')

class DynamicClassTwo(BaseModel):
    model_config = ConfigDict(extra='allow')
    hi: Optional[str] = None
    some_class: Optional["SomeClassNestedDynamic"] = None
    status: Optional[Union[types.DynEnumOne, str]] = None

class DynamicOutput(BaseModel):
    model_config = ConfigDict(extra='allow')

class Earthling(BaseModel):
    age: Checked[Optional[int],Literal["earth_aged", "no_infants"]]

class Education(BaseModel):
    institution: Optional[str] = None
    location: Optional[str] = None
    degree: Optional[str] = None
    major: List[Optional[str]]
    graduation_date: Optional[str] = None

class Email(BaseModel):
    subject: Optional[str] = None
    body: Optional[str] = None
    from_address: Optional[str] = None

class EmailAddress(BaseModel):
    value: Optional[str] = None

class Event(BaseModel):
    title: Optional[str] = None
    date: Optional[str] = None
    location: Optional[str] = None
    description: Optional[str] = None

class FakeImage(BaseModel):
    url: Optional[str] = None

class FlightConfirmation(BaseModel):
    confirmationNumber: Optional[str] = None
    flightNumber: Optional[str] = None
    departureTime: Optional[str] = None
    arrivalTime: Optional[str] = None
    seatNumber: Optional[str] = None

class FooAny(BaseModel):
    planetary_age: Optional[Union["Martian", "Earthling"]] = None
    certainty: Checked[Optional[int],Literal["unreasonably_certain"]]
    species: Checked[Optional[str],Literal["regex_bad", "regex_good", "trivial"]]

class Forest(BaseModel):
    trees: List["Tree"]

class GroceryReceipt(BaseModel):
    receiptId: Optional[str] = None
    storeName: Optional[str] = None
    items: List[Optional[Union[Optional[str], Optional[int], Optional[float]]]]
    totalAmount: Optional[float] = None

class InnerClass(BaseModel):
    prop1: Optional[str] = None
    prop2: Optional[str] = None
    inner: Optional["InnerClass2"] = None

class InnerClass2(BaseModel):
    prop2: Optional[int] = None
    prop3: Optional[float] = None

class InputClass(BaseModel):
    key: Optional[str] = None
    key2: Optional[str] = None

class InputClassNested(BaseModel):
    key: Optional[str] = None
    nested: Optional["InputClass"] = None

class LinkedList(BaseModel):
    head: Optional["Node"] = None
    len: Optional[int] = None

class LiteralClassHello(BaseModel):
    prop: Literal["hello"]

class LiteralClassOne(BaseModel):
    prop: Literal["one"]

class LiteralClassTwo(BaseModel):
    prop: Literal["two"]

class MalformedConstraints(BaseModel):
    foo: Checked[Optional[int],Literal["foo_check"]]

class MalformedConstraints2(BaseModel):
    foo: Optional[int] = None

class Martian(BaseModel):
    """A Martian organism with an age.
    Such a nice type."""
    age: Checked[Optional[int],Literal["young_enough"]]
    """The age of the Martian in Mars years.
    So many Mars years."""

class NamedArgsSingleClass(BaseModel):
    key: Optional[str] = None
    key_two: Optional[bool] = None
    key_three: Optional[int] = None

class Nested(BaseModel):
    prop3: Optional[Union[Optional[str], Optional[None]]] = None
    prop4: Optional[Union[Optional[str], Optional[None]]] = None
    prop20: Optional["Nested2"] = None

class Nested2(BaseModel):
    prop11: Optional[Union[Optional[str], Optional[None]]] = None
    prop12: Optional[Union[Optional[str], Optional[None]]] = None

class NestedBlockConstraint(BaseModel):
    nbc: Checked[Optional["BlockConstraint"],Literal["cross_field"]]

class NestedBlockConstraintForParam(BaseModel):
    nbcfp: Optional["BlockConstraintForParam"] = None

class Node(BaseModel):
    data: Optional[int] = None
    next: Optional["Node"] = None

class OptionalTest_Prop1(BaseModel):
    omega_a: Optional[str] = None
    omega_b: Optional[int] = None

class OptionalTest_ReturnType(BaseModel):
    omega_1: Optional["OptionalTest_Prop1"] = None
    omega_2: Optional[str] = None
    omega_3: List[Optional[types.OptionalTest_CategoryType]]

class OrderInfo(BaseModel):
    order_status: Optional[types.OrderStatus] = None
    tracking_number: Optional[str] = None
    estimated_arrival_date: Optional[str] = None

class OriginalA(BaseModel):
    value: Optional[int] = None

class OriginalB(BaseModel):
    model_config = ConfigDict(extra='allow')
    value: Optional[int] = None

class Person(BaseModel):
    model_config = ConfigDict(extra='allow')
    name: Optional[str] = None
    hair_color: Optional[Union[types.Color, str]] = None

class PhoneNumber(BaseModel):
    value: Optional[str] = None

class Quantity(BaseModel):
    amount: Optional[Union[Optional[int], Optional[float]]] = None
    unit: Optional[str] = None

class RaysData(BaseModel):
    dataType: Optional[types.DataType] = None
    value: Optional[Union["Resume", "Event"]] = None

class ReceiptInfo(BaseModel):
    items: List["ReceiptItem"]
    total_cost: Optional[float] = None
    venue: Optional[Union[Literal["barisa"], Literal["ox_burger"]]] = None

class ReceiptItem(BaseModel):
    name: Optional[str] = None
    description: Optional[str] = None
    quantity: Optional[int] = None
    price: Optional[float] = None

class Recipe(BaseModel):
    ingredients: Dict[str, Optional["Quantity"]]
    recipe_type: Optional[Union[Literal["breakfast"], Literal["dinner"]]] = None

class Resume(BaseModel):
    name: Optional[str] = None
    email: Optional[str] = None
    phone: Optional[str] = None
    experience: List["Education"]
    education: List[Optional[str]]
    skills: List[Optional[str]]

class Schema(BaseModel):
    prop1: Optional[Union[Optional[str], Optional[None]]] = None
    prop2: Optional[Union["Nested", Optional[str]]] = None
    prop5: List[Optional[Union[Optional[str], Optional[None]]]]
    prop6: Optional[Union[Optional[str], List["Nested"]]] = None
    nested_attrs: List[Optional[Union[Optional[str], Optional[None], "Nested"]]]
    parens: Optional[Union[Optional[str], Optional[None]]] = None
    other_group: Optional[Union[Optional[str], Optional[Union[Optional[int], Optional[str]]]]] = None

class SearchParams(BaseModel):
    dateRange: Optional[int] = None
    location: List[Optional[str]]
    jobTitle: Optional["WithReasoning"] = None
    company: Optional["WithReasoning"] = None
    description: List["WithReasoning"]
    tags: List[Optional[Union[Optional[types.Tag], Optional[str]]]]

class SomeClassNestedDynamic(BaseModel):
    model_config = ConfigDict(extra='allow')
    hi: Optional[str] = None

class StringToClassEntry(BaseModel):
    word: Optional[str] = None

class TestClassAlias(BaseModel):
    key: Optional[str] = None
    key2: Optional[str] = None
    key3: Optional[str] = None
    key4: Optional[str] = None
    key5: Optional[str] = None

class TestClassNested(BaseModel):
    prop1: Optional[str] = None
    prop2: Optional["InnerClass"] = None

class TestClassWithEnum(BaseModel):
    prop1: Optional[str] = None
    prop2: Optional[types.EnumInClass] = None

class TestOutputClass(BaseModel):
    prop1: Optional[str] = None
    prop2: StreamState[Optional[int]]

class Tree(BaseModel):
    data: Optional[int] = None
    children: Optional["Forest"] = None

class TwoStoriesOneTitle(BaseModel):
    title: Optional[str] = None
    story_a: Optional[str] = None
    story_b: Optional[str] = None

class UnionTest_ReturnType(BaseModel):
    prop1: Optional[Union[Optional[str], Optional[bool]]] = None
    prop2: List[Optional[Union[Optional[float], Optional[bool]]]]
    prop3: Optional[Union[List[Optional[bool]], List[Optional[int]]]] = None

class WithReasoning(BaseModel):
    value: Optional[str] = None
    reasoning: Optional[str] = None
