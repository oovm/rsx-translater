mod html;

use rsx_translater::RsxBuilder;

#[test]
fn ready() {
    println!("it works!")
}

pub fn html2rsx(input: &str, _: &str, rsx: &str) {
    let mut t = RsxBuilder::default();
    let out = t.html_to_rsx(input).unwrap();
    assert_eq!(out, rsx);
    // let out = t.html_to_rs(input).unwrap();
    // assert_eq!(out, rs);
}