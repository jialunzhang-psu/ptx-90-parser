use super::tree_formatter::truncate_with_ellipsis;
use super::{TreeDisplay, TreeFormatter};
/// TreeDisplay implementations for primitive types that commonly appear in the AST.
use std::fmt;

impl TreeDisplay for String {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("String: \"{}\"", truncate_with_ellipsis(self, 40)))
    }
}

impl TreeDisplay for bool {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("bool: {}", self))
    }
}

impl TreeDisplay for u8 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("u8: {}", self))
    }
}

impl TreeDisplay for u16 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("u16: {}", self))
    }
}

impl TreeDisplay for u32 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("u32: {}", self))
    }
}

impl TreeDisplay for u64 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("u64: {}", self))
    }
}

impl TreeDisplay for i16 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("i16: {}", self))
    }
}

impl TreeDisplay for i32 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("i32: {}", self))
    }
}

impl TreeDisplay for i64 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("i64: {}", self))
    }
}

impl TreeDisplay for i128 {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root(&format!("i128: {}", self))
    }
}

impl TreeDisplay for () {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> fmt::Result {
        f.root("()")
    }
}
