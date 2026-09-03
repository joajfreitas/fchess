#[test]
fn {name}() {{
    let starting_fen: &str = "{starting_fen}";
    let square: Option<&str> = {square:?};
    let expected_moves = {moves:?}.to_vec();

    let board = Board::from_fen(starting_fen).unwrap();
    let move_generator = MoveGenerator::new();

    let movesets: Vec<MoveSet> = if square.is_none() {{
        move_generator.generate_moves(&board)
    }}
    else {{
        vec![move_generator.generate_moves_for_piece(&board, Square::from_algebraic(square.unwrap()).unwrap()).unwrap()]
    }};

    let mut moves: Vec<Move> = movesets.iter().flat_map(|moveset| moveset.into_iter()).collect::<Vec<Move>>();

    let mut expected_moves = expected_moves.into_iter().map(|mov| Move::from_full_algebraic(mov).unwrap()).collect::<Vec<Move>>();

    moves.sort();
    expected_moves.sort();

    assert!(moves == expected_moves);
}}
