pub const path_list_separator:char = if cfg!(windows) {
    ';'
} else {
    ':'
};

pub const path_list_separator_str:&str = if cfg!(windows) {
    ";"
} else {
    ":"
};