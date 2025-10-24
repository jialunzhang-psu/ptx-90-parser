use ptx_parser::{parse_stmt_directive, FunctionStatement, StatementDirective};

#[test]
fn parses_loc_directive() {
    let stmt = parse_stmt_directive(".loc 2 4239 5").expect(".loc should parse");
    match stmt {
        FunctionStatement::Directive(StatementDirective::Loc(loc)) => {
            assert_eq!(loc.file_index, 2);
            assert_eq!(loc.line, 4239);
            assert_eq!(loc.column, 5);
        }
        other => panic!("expected .loc directive, got {:?}", other),
    }
}

#[test]
fn rejects_entry_directive() {
    assert!(parse_stmt_directive(".reg .s32 i;").is_err());
}
