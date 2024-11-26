class Foo {
    bar int @streaming::needed
    baz string | int @streaming::done
    quux bool @streaming::state
}

class Foo2 {
    something int
    tomething_else bool
    @@streaming::done
}