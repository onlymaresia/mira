BEGIN {
    line = ""
    is_parsing = 0
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

{
    if(line == "") {
        line = $0
    }

    sub("::std::option::Option<", "", line)
    print line

    line = ""
    is_parsing = 0
}
