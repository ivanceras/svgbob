#[test]
fn cjk_errors() {
    let input = r#"
    +----+
    | 一 |
    +----+

    +--------+
    |"""一"""|
    +--------+
    "#;

    let out = svgbob::to_svg_string_compressed(input);
    println!("\n{}\n", out);
    dbg!(&out);

    panic!();
}

#[test]
fn german_umlauts() {
    let input = r#"
+----------+
| ÖÄÜ      |
+----------+

+----------+
| "ÖÄÜ"    |
+----------+

+----------+
| "ÖÄÜ" |
+----------+
    "#;

    let out = svgbob::to_svg_string_compressed(input);
    println!("\n{}\n", out);
    dbg!(&out);

    panic!();
}
