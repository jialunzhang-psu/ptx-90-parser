// TreeDisplay implementations for instruction types

use crate::r#type::instruction::Inst;
use super::{TreeDisplay, TreeFormatter};

impl TreeDisplay for Inst {
    fn tree_display(&self, f: &mut TreeFormatter, _source: &str) -> std::fmt::Result {
        // Display debug representation of the instruction
        f.root(&format!("{:?}", self))
    }
}
