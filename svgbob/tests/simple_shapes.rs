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

    let mut settings = Settings::default();
    settings.include_backdrop = false;
    settings.include_styles = false;
    settings.include_defs = false;
    let svg = svgbob::to_svg_with_settings(bob, &settings);
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

    let mut settings = Settings::default();
    settings.include_backdrop = false;
    settings.include_styles = false;
    settings.include_defs = false;
    let svg = svgbob::to_svg_with_settings(bob, &settings);
    println!("{}", svg);
    assert_eq!(expected, svg);
}
