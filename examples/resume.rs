use latex::{Document, Element, Section};
use moderncv::{closesection, document_class, texify, CVPreamble, CVSection, Color, Style};

fn subsection(section: &str) -> Element {
    let s = texify!("subsection", section);
    Element::UserDefined(s)
}

fn pagebreak() -> Element {
    let empty = "";
    let s = texify!("pagebreak", empty);
    Element::UserDefined(s)
}

fn make_preamble(doc: &mut Document) {
    doc.preamble.cvtheme(Style::Casual,Some(Color::Green))
    .firstname("John")
    .familyname("Doe")
    .title("Design enthusiast")
        .address("12 somestreet", Some("3456 somecity"), None)
    .mobile("+123 456 7890")
    .phone("+12 (3)456 78 90",None)
    .email("jdoe@design.org")
    .quote("Any intelligent fool can make things bigger, more complex, and more violent. It takes a touch of genius -- and a lot of courage -- to move in the opposite direction");
}

fn make_sections(doc: &mut Document) {
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
    experience.push(subsection("Vocational"));
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
        );

    experience.push(subsection("Miscellaneous"));

    experience.cventry(
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

    let mut interests = Section::new("Interests");
    interests
        .cvline("hobby 1", r"\small Description")
        .cvline("hobby 2", r"\small Description")
        .cvline("hobby 3", r"\small Description");

    doc.push(interests);
    doc.push(closesection());
    doc.push(pagebreak());

    let mut extra = Section::new("Extra");
    extra
        .cvlistitem("Item 1")
        .cvlistitem("Item 2")
        .cvlistitem("Item 3");

    doc.push(extra);

    let mut extra2 = Section::new("Extra 2");
    extra2
        .cvlistdoubleitem("Item 1", "Item 4")
        .cvlistdoubleitem("Item 2", "Item 5")
        .cvlistdoubleitem("Item 3", "");

    doc.push(extra2);
}

fn main() {
    let mut doc = Document::new(document_class());

    make_preamble(&mut doc);
    doc.push(Element::TitlePage);
    make_sections(&mut doc);

    println!("{}", latex::print(&doc).unwrap());
}
