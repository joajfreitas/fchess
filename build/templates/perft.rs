#[test]
fn {test_name}() {{
    let epd = "{epd}";

    let epd = Epd::from_string(epd).unwrap();

    let d1 = epd.properties.get("D1").unwrap().parse::<i32>().unwrap();

    assert_eq!(d1, 1);
}}
