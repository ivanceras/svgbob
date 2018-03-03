extern crate svgbob;

fn main() {
    let input = r#"
    .-------------------------------------.
    | Hello here and there and everywhere |
    '-------------------------------------'
    "#;
    println!("svg: {}", svgbob::to_svg(input));
}
