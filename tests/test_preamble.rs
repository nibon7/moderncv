use latex::PreambleElement;
use moderncv::preamble::*;

#[test]
fn test_name() {
    let left = name("John", "Doe");
    let right = PreambleElement::UserDefined(r"\name{John}{Doe}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_firstname() {
    let left = firstname("John");
    let right = PreambleElement::UserDefined(r"\firstname{John}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_lastname() {
    let left = lastname("Doe");
    let right = PreambleElement::UserDefined(r"\lastname{Doe}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_givenname() {
    let left = givenname("John");
    let right = PreambleElement::UserDefined(r"\givenname{John}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_familyname() {
    let left = familyname("Doe");
    let right = PreambleElement::UserDefined(r"\familyname{Doe}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_address() {
    let left = address("12 somestreet", Some("3456 somecity"), None);
    let right = PreambleElement::UserDefined(r"\address{12 somestreet}{3456 somecity}".to_string());
    assert!(left.eq(&right));

    let left = address("12 somestreet", None, Some("unused"));
    let right = PreambleElement::UserDefined(r"\address{12 somestreet}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_mobile() {
    let left = mobile("+123 456 7890");
    let right = PreambleElement::UserDefined(r"\mobile{+123 456 7890}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_phone() {
    let left = phone("12 (3)456 78 90", Some(PhoneType::Mobile));
    let right = PreambleElement::UserDefined(r"\phone[mobile]{12 (3)456 78 90}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_fax() {
    let left = fax("12 (3)456 78 90");
    let right = PreambleElement::UserDefined(r"\fax{12 (3)456 78 90}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_email() {
    let left = email("jdoe@design.org");
    let right = PreambleElement::UserDefined(r"\email{jdoe@design.org}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_social() {
    let left = social(r"my\_account", SocialType::Github, None);
    let right = PreambleElement::UserDefined(r"\social[github]{my\_account}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_homepage() {
    let left = homepage(r"https://github.com/my\_home");
    let right = PreambleElement::UserDefined(r"\homepage{https://github.com/my\_home}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_cvtheme() {
    let left = cvtheme(Style::Classic, None);
    let right = PreambleElement::UserDefined(r"\moderncvtheme{classic}".to_string());
    assert!(left.eq(&right));

    let left = cvtheme(Style::Casual, Some(Color::Green));
    let right = PreambleElement::UserDefined(r"\moderncvtheme[green]{casual}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_extrainfo() {
    let left = extrainfo(r"\weblink{www.ctan.org}");
    let right = PreambleElement::UserDefined(r"\extrainfo{\weblink{www.ctan.org}}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_photo() {
    let left = photo("jdoe_picture", Some("64pt"), None);
    let right = PreambleElement::UserDefined(r"\photo[64pt]{jdoe_picture}".to_string());
    assert!(left.eq(&right));
}

#[test]
fn test_quote() {
    let left = quote("Any intelligent fool can make things bigger");
    let right = PreambleElement::UserDefined(
        r"\quote{Any intelligent fool can make things bigger}".to_string(),
    );
    assert!(left.eq(&right));
}
