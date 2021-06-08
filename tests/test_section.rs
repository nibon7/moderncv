use latex::Element;
use moderncv::section::*;

#[test]
fn test_cventry() {
    let left = cventry(
        "2020--2021",
        "Degree",
        "Institution",
        "City",
        Some(r"\textit{Grade}"),
        Some("Description"),
    );

    let right = Element::UserDefined(
        r"\cventry{2020--2021}{Degree}{Institution}{City}{\textit{Grade}}{Description}".to_string(),
    );

    assert!(left.eq(&right));

    let left = cventry(
        "2020--2021",
        "Degree",
        "Institution",
        "City",
        None,
        Some("Description"),
    );

    let right = Element::UserDefined(
        r"\cventry{2020--2021}{Degree}{Institution}{City}{}{Description}".to_string(),
    );

    assert!(left.eq(&right));
}

#[test]
fn test_cvlanguage() {
    let left = cvlanguage("language 1", "Skill level 1", "Comment");

    let right =
        Element::UserDefined(r"\cvlanguage{language 1}{Skill level 1}{Comment}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvcomputer() {
    let left = cvcomputer("category 1", "XXX, YYY, ZZZ", "category 2", "XXX, YYY, ZZZ");

    let right = Element::UserDefined(
        r"\cvcomputer{category 1}{XXX, YYY, ZZZ}{category 2}{XXX, YYY, ZZZ}".to_string(),
    );

    assert!(left.eq(&right));
}

#[test]
fn test_cvline() {
    let left = cvline("hobby 1", r"\small Description");

    let right = Element::UserDefined(r"\cvline{hobby 1}{\small Description}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvdoubleitem() {
    let left = cvdoubleitem("subtitle", "text", "subtitle", "text");

    let right = Element::UserDefined(r"\cvdoubleitem{subtitle}{text}{subtitle}{text}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvlistitem() {
    let left = cvlistitem("Item 1");

    let right = Element::UserDefined(r"\cvlistitem{Item 1}".to_string());

    assert!(left.eq(&right));
}

#[test]
fn test_cvlistdoubleitem() {
    let left = cvlistdoubleitem("Item 1", "Item 4");

    let right = Element::UserDefined(r"\cvlistdoubleitem{Item 1}{Item 4}".to_string());

    assert!(left.eq(&right));
}
