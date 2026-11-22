use svgbob::Settings;

#[test]
fn rect1() {
    let bob = r#"
    +----------+
    |          |
    +----------+
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="136" height="80" class="svgbob">
  <rect x="36" y="24" width="88" height="32" class="solid nofill" rx="0"></rect>
</svg>"#;

    let svg = svgbob::to_svg_with_settings(bob, &Settings::for_debug());
    println!("{}", svg);
    assert_eq!(expected, svg);
}

#[test]
fn escaped_shape() {
    let bob = r#"
    "+----------+"
    "|          |"
    "+----------+"
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="32" class="svgbob">
  <text x="34" y="28" >+----------+</text>
  <text x="34" y="44" >|          |</text>
  <text x="34" y="60" >+----------+</text>
</svg>"#;

    let svg = svgbob::to_svg_with_settings(bob, &Settings::for_debug());
    println!("{}", svg);
    assert_eq!(expected, svg);
}

#[test]
fn rounded_rect() {
    let bob = r#"
    .----------.
    |          |
    '----------'
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="136" height="80" class="svgbob">
  <rect x="36" y="24" width="88" height="32" class="solid nofill" rx="4"></rect>
</svg>"#;

    let svg = svgbob::to_svg_with_settings(bob, &Settings::for_debug());
    println!("{}", svg);
    assert_eq!(expected, svg);
}

#[test]
fn just_v() {
    let bob = r#"
        V
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="80" height="48" class="svgbob">
  <text x="66" y="28" >V</text>
</svg>"#;

    let svg = svgbob::to_svg_with_settings(bob, &Settings::for_debug());
    println!("{}", svg);
    assert_eq!(expected, svg);
}

#[test]
fn arrow_down() {
    let bob = r#"
        |
        V
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="80" height="64" class="svgbob">
  <line x1="68" y1="16" x2="68" y2="36" class="solid"></line>
  <polygon points="64,36 72,36 68,48" class="filled"></polygon>
</svg>"#;

    let svg = svgbob::to_svg_with_settings(bob, &Settings::for_debug());
    println!("{}", svg);
    assert_eq!(expected, svg);
}

#[test]
fn text_with_xml_special_chars() {
    let bob = r#"
    a & b < c > d
    "x & y"
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="144" height="48" class="svgbob">
  <text x="34" y="28" >a</text>
  <text x="50" y="28" >&amp;</text>
  <text x="66" y="28" >b</text>
  <text x="82" y="28" >&lt;</text>
  <text x="98" y="28" >c</text>
  <text x="114" y="28" >&gt;</text>
  <text x="130" y="28" >d</text>
  <text x="34" y="44" >x &amp; y</text>
</svg>"#;

    let svg = svgbob::to_svg_with_settings(bob, &Settings::for_debug());
    println!("{}", svg);
    assert_eq!(expected, svg);
}
