use latex::{Preamble, PreambleElement};
use moderncv::preamble::{CVPreamble, Color, PhoneType, SocialType, Style};

#[test]
fn test_name() {
    let mut preamble = Preamble::default();
    preamble.name("John", "Doe");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\name{John}{Doe}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_firstname() {
    let mut preamble = Preamble::default();
    preamble.firstname("John");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\firstname{John}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_lastname() {
    let mut preamble = Preamble::default();
    preamble.lastname("Doe");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\lastname{Doe}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_givenname() {
    let mut preamble = Preamble::default();
    preamble.givenname("John");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\givenname{John}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_familyname() {
    let mut preamble = Preamble::default();
    preamble.familyname("Doe");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\familyname{Doe}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_address() {
    let mut preamble = Preamble::default();
    preamble.address("12 somestreet", Some("3456 somecity"), None);

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\address{12 somestreet}{3456 somecity}".to_string());
    assert!(left.eq(&right));

    let mut preamble = Preamble::default();
    preamble.address("12 somestreet", None, Some("unused"));

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\address{12 somestreet}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_mobile() {
    let mut preamble = Preamble::default();
    preamble.mobile("+123 456 7890");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\mobile{+123 456 7890}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_phone() {
    let mut preamble = Preamble::default();
    preamble.phone("12 (3)456 78 90", Some(PhoneType::Mobile));

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\phone[mobile]{12 (3)456 78 90}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_fax() {
    let mut preamble = Preamble::default();
    preamble.fax("12 (3)456 78 90");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\fax{12 (3)456 78 90}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_email() {
    let mut preamble = Preamble::default();
    preamble.email("jdoe@design.org");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\email{jdoe@design.org}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_social() {
    let mut preamble = Preamble::default();
    preamble.social(r"my\_account", SocialType::Github, None);

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\social[github]{my\_account}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_homepage() {
    let mut preamble = Preamble::default();
    preamble.homepage(r"https://github.com/my\_home");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\homepage{https://github.com/my\_home}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_cvtheme() {
    let mut preamble = Preamble::default();
    preamble.cvtheme(Style::Classic, None);

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\moderncvtheme{classic}".to_string());
    assert!(left.eq(&right));

    let mut preamble = Preamble::default();
    preamble.cvtheme(Style::Casual, Some(Color::Green));

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\moderncvtheme[green]{casual}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_extrainfo() {
    let mut preamble = Preamble::default();
    preamble.extrainfo(r"\weblink{www.ctan.org}");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\extrainfo{\weblink{www.ctan.org}}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_photo() {
    let mut preamble = Preamble::default();
    preamble.photo("jdoe_picture", Some("64pt"), None);

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(r"\photo[64pt]{jdoe_picture}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_quote() {
    let mut preamble = Preamble::default();
    preamble.quote("Any intelligent fool can make things bigger");

    let left = preamble.iter().nth(0).unwrap();
    let right = PreambleElement::UserDefined(
        r"\quote{Any intelligent fool can make things bigger}".to_string(),
    );
    assert!(left.eq(&right));
}
