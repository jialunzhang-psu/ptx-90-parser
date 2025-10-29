/// `ret{.uni};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ret {
    /// `ret;`
    Default,
    /// `ret.uni;`
    Uniform,
}
