use regex::Regex;
use std::sync::LazyLock;

macro_rules! regex {
    ($name:ident, $re:expr) => {
        pub static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new($re).unwrap()
        });
    };
}

regex!(RE_MOD, r"^\s*(pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;.*$");
regex!(RE_ARGS, r"//.*@(\S+)");
