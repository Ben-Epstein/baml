function TestNamedArgsLiteralBool(myBool: true) -> string {
  client GPT35
  prompt #"
    Return this value back to me: {{myBool}}
  "#
}

test TestFnNamedArgsLiteralBool {
  functions [TestNamedArgsLiteralBool]
  args {
    myBool true
  }
}