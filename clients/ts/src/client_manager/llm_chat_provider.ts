import { FireBamlEvent } from "../ffi_layer";
import { LLMBaseProvider, LLMBaseProviderArgs, LLMChatMessage, LLMResponse } from "./llm_base_provider";
import format from 'string-format';
import { NapiRenderer } from '@boundaryml/baml-core-ffi'

interface LLMChatProviderArgs extends LLMBaseProviderArgs {
  prompt_to_chat: (prompt: string) => LLMChatMessage;
}

abstract class LLMChatProvider extends LLMBaseProvider {
  private prompt_to_chat: (prompt: string) => LLMChatMessage;

  constructor(args: LLMChatProviderArgs) {
    const { prompt_to_chat, ...rest } = args;
    super(rest);
    this.prompt_to_chat = prompt_to_chat;
  }

  protected run_jinja_template_once(jinja_template: string, args: { [key: string]: any; }, output_format: string, template_macros: {
    name: string;
    argNames: string[];
    argTypes: string[];
    template: string;
  }[]): Promise<LLMResponse> {
    let renderer = new NapiRenderer(jinja_template, output_format);
    template_macros.forEach((macro) => {
      renderer.addTemplateString(macro.name, macro.argNames, macro.argTypes, macro.template);
    });
    let env: Record<string, string> = {};
    Object.entries(process.env).forEach(([key, value]) => {
      if (value) {
        env[key] = value;
      }
    });

    const rendered = renderer.render(args, this.napi_client, env);

    if (rendered.isChat()) {
      return this.run_chat_once(rendered.chatMessages().map((chat) => ({
        role: chat.role(),
        content: chat.message(),
      })));
    } else {
      return this.run_prompt_once(rendered.completion());
    }
  }

  run_prompt_once(prompt: string): Promise<LLMResponse> {
    return this.run_chat_once([this.prompt_to_chat(prompt)]);
  }
  run_prompt_template_once(prompt: string, template_args: Array<string>, params: { [key: string]: any; }): Promise<LLMResponse> {
    return this.run_chat_template([this.prompt_to_chat(prompt)], template_args, params);
  }

  run_chat_once(prompt: LLMChatMessage | LLMChatMessage[]): Promise<LLMResponse> {
    const prompts = Array.isArray(prompt) ? prompt : [prompt];
    this.start_run(prompts);
    return this.chat_with_telemetry(prompts);
  }
  run_chat_template_once(prompt: LLMChatMessage | LLMChatMessage[], template_args: Array<string>, params: { [key: string]: any; }): Promise<LLMResponse> {
    const prompts = Array.isArray(prompt) ? prompt : [prompt];

    const updates = template_args.map((arg): [string, string] => [arg, `${params[arg]}`]);

    this.start_run(prompts);
    FireBamlEvent.llmTemplateArgs({
      template: prompts,
      template_args: Object.fromEntries(updates),
    });
    const filled_prompts = prompts.map((prompt) => {
      let content = prompt.content;
      updates.forEach(([arg, value]) => {
        content = content.replaceAll(arg, value);
      });
      return {
        role: prompt.role,
        content,
      }
    });

    return this.chat_with_telemetry(filled_prompts);
  }

  private async chat_with_telemetry(prompt: LLMChatMessage[]): Promise<LLMResponse> {
    try {
      const result = await this.chat_impl(prompt);
      this.end_run(result);
      return result;
    } catch (err) {
      this.raise_error(err);
    }
  }

  /// Method to be implemented by the derived class
  protected abstract chat_impl(prompt: LLMChatMessage[]): Promise<LLMResponse>;
}

export { LLMChatProvider, LLMChatProviderArgs };
