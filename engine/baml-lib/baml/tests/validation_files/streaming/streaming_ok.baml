class Foo {
    bar int @streaming::needed
    baz string | int @stream.done
    quux bool @streaming::state
}

class Foo2 {
    something int
    tomething_else bool
    @@stream.done
}
