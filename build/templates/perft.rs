#[test]
fn {test_name}_d1() -> Result<()> {{
    let epd = "{epd}";

    let epd = Epd::from_string(epd).unwrap();

    let d1 = epd.properties.get("D1").unwrap().parse::<u32>().unwrap();

    let _ = dbg!(epd.board.to_fen());

    assert_eq!(perft(&epd.board, 1)?.0, d1);

    Ok(())
}}

#[test]
fn {test_name}_d2() -> Result<()> {{
    let epd = "{epd}";

    let epd = Epd::from_string(epd).unwrap();

    let d2 = epd.properties.get("D2").unwrap().parse::<u32>().unwrap();

    let _ = dbg!(epd.board.to_fen());

    assert_eq!(perft(&epd.board, 2)?.0, d2);

    Ok(())
}}

#[test]
fn {test_name}_d3() -> Result<()> {{
    let epd = "{epd}";

    let epd = Epd::from_string(epd).unwrap();

    let d3 = epd.properties.get("D3").unwrap().parse::<u32>().unwrap();

    let _ = dbg!(epd.board.to_fen());

    assert_eq!(perft(&epd.board, 3)?.0, d3);

    Ok(())
}}
