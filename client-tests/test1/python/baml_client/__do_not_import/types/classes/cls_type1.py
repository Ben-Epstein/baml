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

from .cls_type2 import Type2
from baml_core._impl.deserializer import register_deserializer
from pydantic import BaseModel
from typing import List


@register_deserializer()
class Type1(BaseModel):
    prop: str
    prop2: int
    propArray: List[str]
    boolean: Type2
