// This file is auto-generated. Do not edit this file manually.
//
// Disable formatting for this file to avoid linting errors.
// tslint:disable
// @ts-nocheck
/* eslint-disable */


import { GPT35 } from '../client';
import { FnOutputBool } from '../function';
import { schema } from '../json_schema';
import { Deserializer } from '@boundaryml/baml-core/deserializer/deserializer';


const prompt_template = `\
Return a bool:\
`;

const deserializer = new Deserializer<boolean>(schema, {
  $ref: '#/definitions/FnOutputBool_output'
});

FnOutputBool.registerImpl('v1', async (
  arg: string
): Promise<boolean> => {
  
    const result = await GPT35.run_prompt_template(
      prompt_template,
      [],
      {
      }
    );

    return deserializer.coerce(result.generated);
  }
);

