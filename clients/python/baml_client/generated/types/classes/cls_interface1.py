# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.0.1
# Generated Date: 2023-10-24 17:14:08.589976 -07:00
# Generated by: aaronvillalpando

from ...._impl.deserializer import register_deserializer
from pydantic import BaseModel


@register_deserializer()
class Interface1(BaseModel):
    prop: String
    prop2: Int
    interface1: Interface1