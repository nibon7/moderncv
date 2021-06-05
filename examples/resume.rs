use latex::{Document, DocumentClass, Element};
use moderncv::*;

fn main() {
    let mut doc = Document::new(DocumentClass::Other("moderncv".to_string()));
    doc.preamble.push(moderncvtheme("casual", Some("green")));
    doc.preamble.push(firstname("John"));
    doc.preamble.push(familyname("Doe"));
    doc.preamble.title("Design enthusiast");
    doc.preamble.push(address("12 somestreet"));
    doc.preamble.push(mobile("+123 456 7890"));
    doc.preamble.push(phone("+12 (3)456 78 90"));
    doc.preamble.push(email("jdoe@design.org"));
    doc.preamble.push(photo("jdoe_picture", Some("64pt")));
    doc.preamble.push(quote("Any intelligent fool can make things bigger, more complex, and more violent. It takes a touch of genius -- and a lot of courage -- to move in the opposite direction"));

    doc.push(Element::TitlePage);

    println!("{}", latex::print(&doc).unwrap());
}
