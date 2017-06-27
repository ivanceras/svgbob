
#[test]
fn char_len(){
    let s = "💖";
    let ch = '💖';
    let chin = "理";
    println!("s: {} {} {}", s, s.len(), s.chars().count());
    println!("ch: {} {}", ch, ch.len_utf8());
    println!("chin: {} {} {} {:?}", chin, chin.len(), s.chars().count(), chin.chars().nth(0).unwrap().len_utf8());
    assert_eq!(ch.len_utf8(), 4);
    assert_eq!(s.len(), 4);
    assert_eq!(s.chars().count(), 1);
    assert_eq!(chin.len(), 3);
    assert_eq!(chin.chars().count(), 1);
}


#[test]
fn test_zero_width(){
    let s = "٩(̾●̮̮̃ ̾•̃̾)۶";
    let s1 = "●̮̮̃";
    assert_eq!(s.len(), 27);
    assert_eq!(s.chars().count(), 14);
    assert_eq!(s1.len(), 9);
    assert_eq!(s1.chars().count(), 4);
}
