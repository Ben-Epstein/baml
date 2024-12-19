class ClassWithDone {
    sixteen_digit_number int
    string_with_twenty_words string @stream.done
}

function MakeClassWithDone() -> ClassWithDone {
    client Claude
    prompt #"
        {{ ctx.output_format }}
    "#
}