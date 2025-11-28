//! Original PTX specification:
//!
//! // Range-based policy
//! createpolicy.range{.global}.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy, [a], primary-size, total-size;
//! // Fraction-based policy
//! createpolicy.fractional.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy{, fraction};
//! // Converting the access property from CUDA APIs
//! createpolicy.cvt.L2.b64            cache-policy, access-property;
//! .level::primary_priority =   { .L2::evict_last, .L2::evict_normal,
//! .L2::evict_first, .L2::evict_unchanged };
//! .level::secondary_priority = { .L2::evict_first, .L2::evict_unchanged };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum LevelPrimaryPriority {
        L2EvictUnchanged, // .L2::evict_unchanged
        L2EvictNormal,    // .L2::evict_normal
        L2EvictFirst,     // .L2::evict_first
        L2EvictLast,      // .L2::evict_last
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum LevelSecondaryPriority {
        L2EvictUnchanged, // .L2::evict_unchanged
        L2EvictFirst,     // .L2::evict_first
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        pub range: (),                                                // .range
        pub global: bool,                                             // {.global}
        pub level_primary_priority: LevelPrimaryPriority,             // .level::primary_priority
        pub level_secondary_priority: Option<LevelSecondaryPriority>, // {.level::secondary_priority}
        pub b64: (),                                                  // .b64
        pub cache_policy: GeneralOperand,                             // cache-policy
        pub a: AddressOperand,                                        // [a]
        pub primary_size: GeneralOperand,                             // primary-size
        pub total_size: GeneralOperand,                               // total-size
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        pub fractional: (),                                           // .fractional
        pub level_primary_priority: LevelPrimaryPriority,             // .level::primary_priority
        pub level_secondary_priority: Option<LevelSecondaryPriority>, // {.level::secondary_priority}
        pub b64: (),                                                  // .b64
        pub cache_policy: GeneralOperand,                             // cache-policy
        pub fraction: Option<GeneralOperand>,                         // {, fraction}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CreatepolicyCvtL2B64 {
        pub cvt: (),                         // .cvt
        pub l2: (),                          // .L2
        pub b64: (),                         // .b64
        pub cache_policy: GeneralOperand,    // cache-policy
        pub access_property: GeneralOperand, // access-property
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CreatepolicyCvtL2B64;
pub use section_0::CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64;
pub use section_0::CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64;
pub use section_0::LevelPrimaryPriority as LevelPrimaryPriority0;
pub use section_0::LevelSecondaryPriority as LevelSecondaryPriority0;
