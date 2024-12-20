class ClassWithDone {
    sixteen_digit_number int
    string_with_twenty_words string @stream.done
    class_1 ClassWithoutDone @stream.done
    class_2 ClassWithBlockDone
    class_done_needed ClassWithBlockDone @stream.not_null
    class_needed ClassWithoutDone @stream.not_null
}

class ClassWithoutDone {
    i_16_digits int
    s_20_words string
}

class ClassWithBlockDone {
    i_16_digits int
    s_20_words string
    @@stream.done
}

function MakeClassWithDone() -> ClassWithDone {
    client Claude
    prompt #"
        {{ ctx.output_format }}
    "#
}