use latex::{Element, Section};
use moderncv::section::CVSection;

#[test]
fn test_cventry() {
    let mut section = Section::new("section");
    section.cventry(
        "2020--2021",
        "Degree",
        "Institution",
        "City",
        Some(r"\textit{Grade}"),
        Some("Description"),
    );

    let left = section.iter().nth(0).unwrap();
    let right = Element::UserDefined(
        r"\cventry{2020--2021}{Degree}{Institution}{City}{\textit{Grade}}{Description}".to_string(),
    );

    assert!(left.eq(&right));

    let mut section = Section::new("section");
    section.cventry(
        "2020--2021",
        "Degree",
        "Institution",
        "City",
        None,
        Some("Description"),
    );

    let left = section.iter().nth(0).unwrap();
    let right = Element::UserDefined(
        r"\cventry{2020--2021}{Degree}{Institution}{City}{}{Description}".to_string(),
    );

    assert!(left.eq(&right));
}

#[test]
fn test_cvlanguage() {
    let mut section = Section::new("section");
    section.cvlanguage("language 1", "Skill level 1", "Comment");

    let left = section.iter().nth(0).unwrap();
    let right =
        Element::UserDefined(r"\cvlanguage{language 1}{Skill level 1}{Comment}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvcomputer() {
    let mut section = Section::new("section");
    section.cvcomputer("category 1", "XXX, YYY, ZZZ", "category 2", "XXX, YYY, ZZZ");

    let left = section.iter().nth(0).unwrap();
    let right = Element::UserDefined(
        r"\cvcomputer{category 1}{XXX, YYY, ZZZ}{category 2}{XXX, YYY, ZZZ}".to_string(),
    );

    assert!(left.eq(&right));
}

#[test]
fn test_cvline() {
    let mut section = Section::new("section");
    section.cvline("hobby 1", r"\small Description");

    let left = section.iter().nth(0).unwrap();
    let right = Element::UserDefined(r"\cvline{hobby 1}{\small Description}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvdoubleitem() {
    let mut section = Section::new("section");
    section.cvdoubleitem("subtitle", "text", "subtitle", "text");

    let left = section.iter().nth(0).unwrap();
    let right = Element::UserDefined(r"\cvdoubleitem{subtitle}{text}{subtitle}{text}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvlistitem() {
    let mut section = Section::new("section");
    section.cvlistitem("Item 1");

    let left = section.iter().nth(0).unwrap();
    let right = Element::UserDefined(r"\cvlistitem{Item 1}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvlistdoubleitem() {
    let mut section = Section::new("section");
    section.cvlistdoubleitem("Item 1", "Item 4");

    let left = section.iter().nth(0).unwrap();
    let right = Element::UserDefined(r"\cvlistdoubleitem{Item 1}{Item 4}".to_string());

    assert!(left.eq(&right));
}
