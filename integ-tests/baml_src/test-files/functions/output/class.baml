class TestOutputClass {
  prop1 string @description("A long string with about 20 words") @streaming::done @streaming::state
  prop2 int @streaming::state
}

function FnOutputClass(input: string) -> TestOutputClass {
  client GPT35
  prompt #"
    Return a JSON blob with this schema: 
    {{ctx.output_format}}

    For the prop2, always return a 540

    JSON:
  "#
}

test TestClass {
  functions [FnOutputClass]
  args {
    input "example input"
  }
}
