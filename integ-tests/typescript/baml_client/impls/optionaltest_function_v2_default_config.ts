// This file is auto-generated. Do not edit this file manually.
//
// Disable formatting for this file to avoid linting errors.
// tslint:disable
// @ts-nocheck
/* eslint-disable */


import { GPT35 } from '../client';
import { OptionalTest_Function_V2 } from '../function';
import { schema } from '../json_schema';
import { Deserializer } from '@boundaryml/baml-core/deserializer/deserializer';


// Impl: default_config
// Client: GPT35
// An implementation for OptionalTest_Function_V2


const prompt_template = `

Return a JSON blob with this schema: 
{#print_type(output)}
Here's a list of values you can use for
{#print_enum(OptionalTest_CategoryTypev2)}

JSON:`;
const output_format = `({
  "omega_1": {
    "omega_a": string,
    "omega_b": int
  } | null,
  "omega_2": string | null,
  "omega_3": ("OptionalTest_CategoryTypev2 as string" | null)[]
} | null)[]

OptionalTest_CategoryTypev2
---
Aleph
Beta
Gamma`;

const template_macros = [
]

const deserializer = new Deserializer<OptionalTest_ReturnTypev2 | null[]>(schema, {
  $ref: '#/definitions/OptionalTest_Function_V2_output'
});

OptionalTest_Function_V2.registerImpl('default_config', async (
  args: {
    input: string
  }
): Promise<OptionalTest_ReturnTypev2 | null[]> => {
    const result = await GPT35.run_jinja_template(
      prompt_template,
      args,
      output_format,
      template_macros,
    );

    return deserializer.coerce(result.generated);
  }
);

