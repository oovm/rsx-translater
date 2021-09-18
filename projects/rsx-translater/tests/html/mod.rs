use super::*;


#[test]
fn test_radio() {
    html2rsx(include_str!("radio.html"), include_str!("radio.g.rs"), include_str!("radio.x.rs"))
}