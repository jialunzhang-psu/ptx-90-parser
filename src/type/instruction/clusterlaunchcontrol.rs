use crate::r#type::common::AddressOperand;

/// `clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast::cluster::all}.b128 [addr], [mbar];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryCancel {
    /// `.space`
    pub space: Option<Space>,
    /// `.completion_mechanism`
    pub completion_mechanism: CompletionMechanism,
    /// `.multicast::cluster::all`
    pub multicast: Option<Multicast>,
    /// `.b128`
    pub data_type: DataType,
    /// `[addr]`
    pub address: AddressOperand,
    /// `[mbar]`
    pub mbarrier: AddressOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    /// `.shared::cta`
    SharedCta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionMechanism {
    /// `.mbarrier::complete_tx::bytes`
    MbarrierCompleteTxBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b128`
    B128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multicast {
    /// `.multicast::cluster::all`
    ClusterAll,
}
