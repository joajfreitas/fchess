#[test]
fn {name}() {{
    let starting_fen: &str = "{starting_fen}";
    let lan: &str = "{lan}";
    let expected_fen = "{expected_fen}";

    let board = Board::from_fen(starting_fen).unwrap();
    let mov = Move::from_algebraic(lan).unwrap();

    let resulting_board = board.apply(&mov).unwrap();
    let expected_board = Board::from_fen(expected_fen).unwrap();

    assert!(resulting_board == expected_board);
}}
