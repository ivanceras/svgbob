use svgbob::Settings;

#[test]
fn rect1() {
    let bob = r#"
    +----------+
    |          |
    +----------+
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="136" height="80">
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

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="32">
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

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="136" height="80">
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

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="80" height="48">
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

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="80" height="64">
  <line x1="68" y1="16" x2="68" y2="36" class="solid"></line>
  <polygon points="64,36 72,36 68,48" class="filled"></polygon>
</svg>"#;

    let svg = svgbob::to_svg_with_settings(bob, &Settings::for_debug());
    println!("{}", svg);
    assert_eq!(expected, svg);
}
