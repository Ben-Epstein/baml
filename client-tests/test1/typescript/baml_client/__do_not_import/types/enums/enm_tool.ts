// This file is generated by the BAML compiler.
// Do not edit this file directly.
// Instead, edit the BAML files and recompile.

// eslint-disable-next-line @typescript-eslint/no-unused-vars
// @ts-nocheck
// @ts-ignore
// prettier-ignore

from baml_lib._impl.deserializer import register_deserializer
from enum import Enum


@register_deserializer({ "k1": "CodeInterpreter","k2": "DrawImage","k3": "GenerateText", })
class Tool(str, Enum):
    CodeInterpreter = "CodeInterpreter"
    DrawImage = "DrawImage"
    GenerateText = "GenerateText"