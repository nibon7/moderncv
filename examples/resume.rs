use latex::{Document, Element};
use moderncv::{document_class, CVPreamble};

fn main() {
    let mut doc = Document::new(document_class());
    doc.preamble.cvtheme("casual", Some("green"))
    .firstname("John")
    .familyname("Doe")
    .title("Design enthusiast")
        .address("12 somestreet", Some("3456 somecity"), None)
    .mobile("+123 456 7890")
    .phone("+12 (3)456 78 90")
    .email("jdoe@design.org")
    .photo("jdoe_picture", Some("64pt"), None)
    .quote("Any intelligent fool can make things bigger, more complex, and more violent. It takes a touch of genius -- and a lot of courage -- to move in the opposite direction");

    doc.push(Element::TitlePage);

    println!("{}", latex::print(&doc).unwrap());
}
