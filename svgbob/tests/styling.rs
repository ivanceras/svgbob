#[test]

fn test_styling() {
    let bob = r#"
.----------.   .----------.
|{w} A     |-->|{w} B     |
'----------'   '----------'

.----------.   .----------.
|{w} A     |<--|{w} B     |
'----------'   '----------'

.----------.   .----------.
|{w} A     |<->|{w} B     |
'----------'   '----------'

# Legend:
w = {
 fill: #abadb0;
}
"#;

    let svg = svgbob::to_svg_string_compressed(bob);

    println!("{}", &svg);
    panic!();
}
