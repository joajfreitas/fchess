#[test]
fn {name}() {{
    let expected_moves = {expected_moves_san};
    let board = Board::from_fen("{starting_fen}").unwrap();

    let mut expected_moves = expected_moves
        .into_iter()
        .map(|mov| Move::from_san(mov,&board)
            .expect(&format!("Can't read expected SAN {{}}", mov)))
        .collect::<Vec<Move>>();

    let mut moves = MoveGenerator::new()
        .generate_moves(&board)
        .into_iter()
        .flat_map(|moveset| moveset.into_iter().collect::<Vec<Move>>())
        .collect::<Vec<Move>>();

    moves.sort();
    expected_moves.sort();

    assert_that!(moves, eq(&expected_moves), "{starting_fen}");
}}
