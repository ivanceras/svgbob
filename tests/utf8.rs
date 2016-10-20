
#[test]
fn char_len(){
    let s = "💖";
    let ch = '💖';
    println!("s: {} {} {}", s, s.len(), s.chars().count());
    println!("ch: {} {}", ch, ch.len_utf8());
}
