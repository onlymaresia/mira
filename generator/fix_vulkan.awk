BEGIN {
    proc_option = 0
    skip_curr_line = 0
}

/::std::option::Option</ {
    line = $0
    sub("::std::option::Option<", "", line)

    sub(">;", ";", line)

    print line
    skip_curr_line = 1
}

# ) -> ???,
/)[ \t]*[->]?[ \t]*[^,]*,$/ {
    line = $0
    sub(",$", ";", line)

    getline
    ahead = $0
        
    if (match(ahead, />,/)) {
        sub(";$", ",", line)
        ahead = ""
    } else {
        sub(">;", "", ahead)
    }

    print line
    print ahead

    skip_curr_line = 1
}

{
    if(skip_curr_line == 1) {
        skip_curr_line = 0
    } else {
        line = $0
        sub("pub const Vk[a-zA-Z0-9]+_", "pub const ", line)
        print line
    }
}