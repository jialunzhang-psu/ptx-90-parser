// FIXME: These tests need parser implementation for Bar/Barrier instructions
// The type definitions exist but parser may not be complete

#[test]
#[ignore]
fn parses_barrier_sync_with_aligned_and_count() {
    // TODO: Implement parser for Barrier1
}

#[test]
#[ignore]
fn parses_barrier_reduction_logical_with_count() {
    // TODO: Implement parser for Barrier4
}

#[test]
#[ignore]
fn parses_bar_reduction_popc_without_count() {
    // TODO: Implement parser for Bar3
}

#[test]
#[ignore]
fn parses_bar_sync_without_count() {
    // TODO: Implement parser for Bar1
}

#[test]
#[ignore]
fn rejects_barrier_arrive_without_count() {
    // TODO: Implement parser for Barrier2
}

#[test]
#[ignore]
fn rejects_bar_reduction_with_aligned_modifier() {
    // TODO: Implement parser for Bar3
}
