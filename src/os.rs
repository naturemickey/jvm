pub const path_list_separator:char = if cfg!(windows) {
    ';'
} else {
    ':'
};