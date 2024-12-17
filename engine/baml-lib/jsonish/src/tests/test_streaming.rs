use super::*;

const NUMBERS: &str = r#"
class Foo {
  nums int[]
}
"#;

test_partial_deserializer_streaming!(
    test_number_list,
    NUMBERS,
    "{'nums': [1,2",
    FieldType::class("Foo"),
    {"nums": [1]}
);


const NUMBERS_STATE: &str = r#"
class Foo {
  nums int[] @streaming::state
}
"#;

test_partial_deserializer_streaming!(
    test_number_list_state_incomplete,
    NUMBERS_STATE,
    "{'nums': [1,2",
    FieldType::class("Foo"),
    {"nums": {"value": [1], "completion_state": "Incomplete"}}
);

const TOPLEVEL_DONE: &str = r#"
class Foo {
  nums int[]
  @@streaming::done
}
"#;

test_partial_deserializer_streaming_failure!(
  test_toplevel_done,
  TOPLEVEL_DONE,
  "{'nums': [1,2]",
  FieldType::class("Foo")
);

const NESTED_DONE: &str = r#"
class Foo {
  nums int[]
  @@streaming::done
}

class Bar {
  foos Foo[]
}
"#;

test_partial_deserializer_streaming!(
  test_nested_done,
  NESTED_DONE,
  r#"{
    'foos': [
      {'nums': [1, 2]},
      {'nums': [3, 4]
  "#,
  FieldType::class("Bar"),
  {"foos": [ {"nums": [1, 2]}]}
);

const NESTED_DONE_WITH_TOPLEVEL_DONE: &str = r#"
class Foo {
  nums int[]
  @@streaming::done
}

class Bar {
  message string @streaming::done
  foos Foo[]
}
"#;

test_partial_deserializer_streaming!(
  test_nested_done_with_toplevel_done,
  NESTED_DONE_WITH_TOPLEVEL_DONE,
  r#"{
    'message': "Hello",
    'foos': [
      {'nums': [1, 2]},
      {'nums': [3, 4]
  "#,
  FieldType::class("Bar"),
  {"message": "Hello", "foos": [ {"nums": [1, 2]}]}
);

const NEEDED_FIELD: &str = r#"
class Foo {
  my_int int
  my_string string @streaming::needed
}

class Bar {
  foos Foo[]
}
"#;

test_partial_deserializer_streaming!(
  test_needed_field,
  NEEDED_FIELD,
  r#"{"foos": [{"my_int": 1, "my_string": "hi"}, {"my_int": 10,"#,
  FieldType::class("Bar"),
  {"foos": [ {"my_int": 1, "my_string": "hi"} ]}
);

const DONE_FIELD: &str = r#"
class Foo {
  foo string @streaming::done
  bar string
}
"#;

test_partial_deserializer_streaming!(
  test_done_field,
  DONE_FIELD,
  r#"{"foo": ""#,
  FieldType::class("Foo"),
  {"foo": null, "bar": null}
);