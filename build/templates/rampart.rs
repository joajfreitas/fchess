#[test]
fn {name}() {{
    let board = Board::from_fen("{starting_fen}").unwrap();
    let move_generator = MoveGenerator::new();

    let moves = move_generator.generate_moves(&board);

    let expected_mov = Move::from_san("{san}", &board).unwrap();

    let mut found_move = false;
    for mov in moves {{
        if mov.contains(&expected_mov){{
            found_move = true;
            break;
        }}
    }}

    assert!(found_move, "Expected move not found in generated moves");
}}
