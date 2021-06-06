use latex::{Document, DocumentClass, Element};
use moderncv::CVPreamble;

fn main() {
    let mut doc = Document::new(DocumentClass::Other("moderncv".to_string()));
    doc.preamble.theme("casual", Some("green"));
    doc.preamble.firstname("John");
    doc.preamble.familyname("Doe");
    doc.preamble.title("Design enthusiast");
    doc.preamble
        .address("12 somestreet", Some("3456 somecity"), None);
    doc.preamble.mobile("+123 456 7890");
    doc.preamble.phone("+12 (3)456 78 90");
    doc.preamble.email("jdoe@design.org");
    doc.preamble.photo("jdoe_picture", Some("64pt"), None);
    doc.preamble.quote("Any intelligent fool can make things bigger, more complex, and more violent. It takes a touch of genius -- and a lot of courage -- to move in the opposite direction");

    doc.push(Element::TitlePage);

    println!("{}", latex::print(&doc).unwrap());
}
