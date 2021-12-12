BEGIN {
    line = ""
    is_parsing = 0
    remove = 0
    reset_remove = 0
}

/)[ \t]*[->]?[ \t]*[^,]*,$/ {
    if(!is_parsing) {
        line = $0
        is_parsing = 1
    }

    sub(",$", "", line)
}

/>;/ {
    if(!is_parsing) {
        line = $0
        is_parsing = 1
    }

    sub(">;", ";", line)
}

/^extern "C" {/ {
    remove = 1
}

/}$/ {
    reset_remove = 1
}

{
    if(line == "") {
        line = $0
    }

    sub("::std::option::Option<", "", line)
    sub("pub const Vk[a-zA-Z0-9]+_", "pub const ", line)

    if(!remove) {
        print line
    }

    if(reset_remove) {
        reset_remove = remove = 0
    }

    line = ""
    is_parsing = 0
}
