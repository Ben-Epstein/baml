// This file is auto-generated. Do not edit this file manually.
//
// Disable formatting for this file to avoid linting errors.
// tslint:disable
// @ts-nocheck
/* eslint-disable */


import { GPT35 } from '../client';
import { V2_FnEnumListOutput } from '../function';
import { schema } from '../json_schema';
import { Deserializer } from '@boundaryml/baml-core/deserializer/deserializer';


// Impl: default_config
// Client: GPT35
// An implementation for V2_FnEnumListOutput


const prompt_template = `Print out two of these values randomly selected from the list below in a json array.

{{ ctx.output_format }}

Answer:`;
const output_format = `"VALUE_ENUM as string"[]

VALUE_ENUM
---
ONE
TWO
THREE`;

const template_macros = [
]

const deserializer = new Deserializer<EnumOutput[]>(schema, {
  $ref: '#/definitions/V2_FnEnumListOutput_output'
});

V2_FnEnumListOutput.registerImpl('default_config', async (
  args: {
    input: string
  }
): Promise<EnumOutput[]> => {
    const result = await GPT35.run_jinja_template(
      prompt_template,
      args,
      output_format,
      template_macros,
    );

    return deserializer.coerce(result.generated);
  }
);

