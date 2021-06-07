//! An ergonomic library for programatically generating LaTeX resume.

use latex::{DocumentClass, Preamble, PreambleElement};

/// TeXify strings
#[macro_export]
macro_rules! texify {
    ($name:expr) => {{
        format!(r"\{}", $name)
    }};
    ($name:expr $(,[$opt:ident])* $(,$extra:ident)+) => {{
        let mut s = texify!($name);

        $(
            if let Some(opt) = $opt {
                s.push_str(&format!(r"[{}]", opt));
            }
        )*

        $(
            s.push_str(&format!(r"{{{}}}", $extra));
        )+
            s
    }};
}

/// DocumentClass for moderncv
pub fn document_class() -> DocumentClass {
    DocumentClass::Other("moderncv".to_string())
}

/// Social media types
pub enum SocialType {
    LinkedIn,
    XING,
    Twitter,
    Github,
    Gitlab,
    StackOverflow,
    Bitbucket,
    Skype,
    ORCID,
    ResearchGate,
    ResearcherID,
    Telegram,
    GoogleScholar,
}

impl std::fmt::Display for SocialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::LinkedIn => write!(f, "linkedin"),
            Self::XING => write!(f, "xing"),
            Self::Twitter => write!(f, "twitter"),
            Self::Github => write!(f, "github"),
            Self::Gitlab => write!(f, "gitlab"),
            Self::StackOverflow => write!(f, "stackoverflow"),
            Self::Bitbucket => write!(f, "bitbucket"),
            Self::Skype => write!(f, "skype"),
            Self::ORCID => write!(f, "orcid"),
            Self::ResearchGate => write!(f, "researchgate"),
            Self::ResearcherID => write!(f, "researcherid"),
            Self::Telegram => write!(f, "telegram"),
            Self::GoogleScholar => write!(f, "googlescholar"),
        }
    }
}

/// Phone types
pub enum PhoneType {
    Fixed,
    Mobile,
    Fax,
}

impl std::fmt::Display for PhoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Fixed => write!(f, "fixed"),
            Self::Mobile => write!(f, "mobile"),
            Self::Fax => write!(f, "fax"),
        }
    }
}

/// Moderncv colors
pub enum Color {
    Black,
    Blue,
    Burgundy,
    Green,
    Grey,
    Orange,
    Purple,
    Red,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Black => write!(f, "black"),
            Self::Blue => write!(f, "blue"),
            Self::Burgundy => write!(f, "burgundy"),
            Self::Green => write!(f, "green"),
            Self::Grey => write!(f, "grey"),
            Self::Orange => write!(f, "orange"),
            Self::Purple => write!(f, "purple"),
            Self::Red => write!(f, "red"),
        }
    }
}

/// Moderncv styles
pub enum Style {
    Banking,
    Casual,
    Classic,
    Empty,
    Fancy,
    OldStyle,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Banking => write!(f, "banking"),
            Self::Casual => write!(f, "casual"),
            Self::Classic => write!(f, "classic"),
            Self::Empty => write!(f, "empty"),
            Self::Fancy => write!(f, "fancy"),
            Self::OldStyle => write!(f, "oldstyle"),
        }
    }
}

/// A trait which implement useful functions for moderncv preamble
pub trait CVPreamble {
    fn name(&mut self, firstname: &str, lastname: &str) -> &mut Self;
    fn firstname(&mut self, fistname: &str) -> &mut Self;
    fn lastname(&mut self, lastname: &str) -> &mut Self;
    fn givenname(&mut self, givenname: &str) -> &mut Self;
    fn familyname(&mut self, familyname: &str) -> &mut Self;
    fn address(&mut self, street: &str, city: Option<&str>, country: Option<&str>) -> &mut Self;
    fn mobile(&mut self, mobile: &str) -> &mut Self;
    fn phone(&mut self, number: &str, phone_type: Option<PhoneType>) -> &mut Self;
    fn fax(&mut self, fax: &str) -> &mut Self;
    fn email(&mut self, email: &str) -> &mut Self;
    fn social(&mut self, account: &str, social_type: SocialType, url: Option<&str>) -> &mut Self;
    fn homepage(&mut self, homepage: &str) -> &mut Self;
    fn cvtheme(&mut self, style: Style, color: Option<Color>) -> &mut Self;
    fn extrainfo(&mut self, info: &str) -> &mut Self;
    fn photo(&mut self, photo: &str, width: Option<&str>, frame: Option<&str>) -> &mut Self;
    fn quote(&mut self, quote: &str) -> &mut Self;
}

impl CVPreamble for Preamble {
    /// Set name
    fn name(&mut self, firstname: &str, lastname: &str) -> &mut Self {
        let s = texify!("name", firstname, lastname);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set first name
    fn firstname(&mut self, firstname: &str) -> &mut Self {
        let s = texify!("firstname", firstname);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set last name
    fn lastname(&mut self, lastname: &str) -> &mut Self {
        let s = texify!("lastname", lastname);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set given name (First name)
    fn givenname(&mut self, givenname: &str) -> &mut Self {
        let s = texify!("givenname", givenname);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set family name (Last name)
    fn familyname(&mut self, familyname: &str) -> &mut Self {
        let s = texify!("familyname", familyname);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set address
    fn address(&mut self, street: &str, city: Option<&str>, country: Option<&str>) -> &mut Self {
        let mut s = texify!("address", street);
        match city {
            Some(city) => {
                s.push('{');
                s.push_str(city);
                s.push('}');
            }
            None => {
                let elem = PreambleElement::UserDefined(s);
                self.push(elem);

                return self;
            }
        }

        if let Some(country) = country {
            s.push('{');
            s.push_str(country);
            s.push('}');
        }

        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set mobile number
    fn mobile(&mut self, mobile: &str) -> &mut Self {
        let s = texify!("mobile", mobile);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set phone number
    fn phone(&mut self, number: &str, phone_type: Option<PhoneType>) -> &mut Self {
        let s = texify!("phone", [phone_type], number);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set fax number
    fn fax(&mut self, fax: &str) -> &mut Self {
        let s = texify!("fax", fax);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set email address
    fn email(&mut self, email: &str) -> &mut Self {
        let s = texify!("email", email);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set social link
    fn social(&mut self, account: &str, social_type: SocialType, url: Option<&str>) -> &mut Self {
        let mut s = texify!("social");
        s.push_str(&format!(r"[{}]", social_type));

        if let Some(url) = url {
            s.push_str(&format!(r"[{}]", url));
        }

        s.push_str(&format!(r"{{{}}}", account));

        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set home page
    fn homepage(&mut self, homepage: &str) -> &mut Self {
        let s = texify!("homepage", homepage);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set moderncv theme
    fn cvtheme(&mut self, style: Style, color: Option<Color>) -> &mut Self {
        let s = texify!("moderncvtheme", [color], style);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set extra information
    fn extrainfo(&mut self, info: &str) -> &mut Self {
        let s = texify!("extrainfo", info);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set photo
    fn photo(&mut self, photo: &str, width: Option<&str>, frame: Option<&str>) -> &mut Self {
        let s = texify!("photo", [width], [frame], photo);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }

    /// Set quote string
    fn quote(&mut self, quote: &str) -> &mut Self {
        let s = texify!("quote", quote);
        let elem = PreambleElement::UserDefined(s);
        self.push(elem);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let right =
            PreambleElement::UserDefined(r"\address{12 somestreet}{3456 somecity}".to_string());
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
        let right =
            PreambleElement::UserDefined(r"\homepage{https://github.com/my\_home}".to_string());
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
}
