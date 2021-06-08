use latex::{Document, Element, Section};
use moderncv::preamble::{Color, Style};
use moderncv::{document_class, CVPreamble, CVSection};

fn main() {
    let mut doc = Document::new(document_class());
    doc.preamble.cvtheme(Style::Casual,Some(Color::Green))
    .firstname("John")
    .familyname("Doe")
    .title("Design enthusiast")
        .address("12 somestreet", Some("3456 somecity"), None)
    .mobile("+123 456 7890")
    .phone("+12 (3)456 78 90",None)
    .email("jdoe@design.org")
    .quote("Any intelligent fool can make things bigger, more complex, and more violent. It takes a touch of genius -- and a lot of courage -- to move in the opposite direction");

    doc.push(Element::TitlePage);

    let mut education = Section::new("Education");
    education
        .cventry(
            "year--year",
            "Degree",
            "Institution",
            "City",
            Some(r"\textit{Grade}"),
            Some("Description"),
        )
        .cventry(
            "year--year",
            "Degree",
            "Institution",
            "City",
            Some(r"\textit{Grade}"),
            Some("Description"),
        );

    doc.push(education);

    let mut thesis = Section::new("Master thesis");
    thesis
        .cvline("title", r"\emph{Title}")
        .cvline("supervisors", "Supervisors")
        .cvline("description", r"\small Short thesis abstract");

    doc.push(thesis);

    let mut experience = Section::new("Experience");
    experience
        .cventry(
            "year--year",
            "Job title",
            "Employer",
            "City",
            None,
            Some("Description"),
        )
        .cventry(
            "year--year",
            "Job title",
            "Employer",
            "City",
            None,
            Some("Description"),
        )
        .cventry(
            "year--year",
            "Job title",
            "Employer",
            "City",
            None,
            Some(r"Description line 1\newline{}Description line 2"),
        );

    doc.push(experience);

    let mut languages = Section::new("Languages");
    languages
        .cvlanguage("language 1", "Skill level", "Comment")
        .cvlanguage("language 2", "Skill level", "Comment");

    doc.push(languages);

    let mut computer = Section::new("Computer skills");
    computer
        .cvcomputer("category 1", "XXX, YYY, ZZZ", "category 3", "XXX, YYY, ZZZ")
        .cvcomputer("category 2", "XXX, YYY, ZZZ", "category 4", "XXX, YYY, ZZZ");

    doc.push(computer);

    println!("{}", latex::print(&doc).unwrap());
}
