#[test]
fn {test_name}_d1() {{
    let epd = "{epd}";

    let epd = Epd::from_string(epd).unwrap();

    let d1 = epd.properties.get("D1").unwrap().parse::<u32>().unwrap();

    dbg!(epd.board.to_fen());

    assert_eq!(perft(&epd.board, 1), Some(d1));
}}

#[test]
fn {test_name}_d2() {{
    let epd = "{epd}";

    let epd = Epd::from_string(epd).unwrap();

    let d2 = epd.properties.get("D2").unwrap().parse::<u32>().unwrap();

    dbg!(epd.board.to_fen());

    assert_eq!(perft(&epd.board, 2), Some(d2));
}}

#[test]
fn {test_name}_d3() {{
    let epd = "{epd}";

    let epd = Epd::from_string(epd).unwrap();

    let d3 = epd.properties.get("D3").unwrap().parse::<u32>().unwrap();

    dbg!(epd.board.to_fen());

    assert_eq!(perft(&epd.board, 3), Some(d3));
}}
