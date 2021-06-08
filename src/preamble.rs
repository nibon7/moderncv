use super::texify;
use latex::{Preamble, PreambleElement};

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
    fn firstname(&mut self, name: &str) -> &mut Self;
    fn lastname(&mut self, name: &str) -> &mut Self;
    fn givenname(&mut self, name: &str) -> &mut Self;
    fn familyname(&mut self, name: &str) -> &mut Self;
    fn address(&mut self, street: &str, city: Option<&str>, country: Option<&str>) -> &mut Self;
    fn mobile(&mut self, number: &str) -> &mut Self;
    fn phone(&mut self, number: &str, phone_type: Option<PhoneType>) -> &mut Self;
    fn fax(&mut self, number: &str) -> &mut Self;
    fn email(&mut self, address: &str) -> &mut Self;
    fn social(&mut self, account: &str, social_type: SocialType, url: Option<&str>) -> &mut Self;
    fn homepage(&mut self, url: &str) -> &mut Self;
    fn cvtheme(&mut self, style: Style, color: Option<Color>) -> &mut Self;
    fn extrainfo(&mut self, info: &str) -> &mut Self;
    fn photo(&mut self, photo: &str, width: Option<&str>, frame: Option<&str>) -> &mut Self;
    fn quote(&mut self, msg: &str) -> &mut Self;
}

impl CVPreamble for Preamble {
    fn name(&mut self, firstname: &str, lastname: &str) -> &mut Self {
        let elem = self::name(firstname, lastname);
        self.push(elem);

        self
    }

    fn firstname(&mut self, name: &str) -> &mut Self {
        let elem = self::firstname(name);
        self.push(elem);

        self
    }

    fn lastname(&mut self, name: &str) -> &mut Self {
        let elem = self::lastname(name);
        self.push(elem);

        self
    }

    fn givenname(&mut self, name: &str) -> &mut Self {
        let elem = self::givenname(name);
        self.push(elem);

        self
    }

    fn familyname(&mut self, name: &str) -> &mut Self {
        let elem = self::familyname(name);
        self.push(elem);

        self
    }

    fn address(&mut self, street: &str, city: Option<&str>, country: Option<&str>) -> &mut Self {
        let elem = self::address(street, city, country);
        self.push(elem);

        self
    }

    fn mobile(&mut self, number: &str) -> &mut Self {
        let elem = self::mobile(number);
        self.push(elem);

        self
    }

    fn phone(&mut self, number: &str, phone_type: Option<PhoneType>) -> &mut Self {
        let elem = self::phone(number, phone_type);
        self.push(elem);

        self
    }

    fn fax(&mut self, number: &str) -> &mut Self {
        let elem = self::fax(number);
        self.push(elem);

        self
    }

    fn email(&mut self, address: &str) -> &mut Self {
        let elem = self::email(address);
        self.push(elem);

        self
    }

    fn social(&mut self, account: &str, social_type: SocialType, url: Option<&str>) -> &mut Self {
        let elem = self::social(account, social_type, url);
        self.push(elem);

        self
    }

    fn homepage(&mut self, url: &str) -> &mut Self {
        let elem = self::homepage(url);
        self.push(elem);

        self
    }

    fn cvtheme(&mut self, style: Style, color: Option<Color>) -> &mut Self {
        let elem = self::cvtheme(style, color);
        self.push(elem);

        self
    }

    fn extrainfo(&mut self, info: &str) -> &mut Self {
        let elem = self::extrainfo(info);
        self.push(elem);

        self
    }

    fn photo(&mut self, photo: &str, width: Option<&str>, frame: Option<&str>) -> &mut Self {
        let elem = self::photo(photo, width, frame);
        self.push(elem);

        self
    }

    fn quote(&mut self, msg: &str) -> &mut Self {
        let elem = self::quote(msg);
        self.push(elem);

        self
    }
}

/// Set name
pub fn name(firstname: &str, lastname: &str) -> PreambleElement {
    let s = texify!("name", firstname, lastname);
    PreambleElement::UserDefined(s)
}

/// Set first name
pub fn firstname(name: &str) -> PreambleElement {
    let s = texify!("firstname", name);
    PreambleElement::UserDefined(s)
}

/// Set last name
pub fn lastname(name: &str) -> PreambleElement {
    let s = texify!("lastname", name);
    PreambleElement::UserDefined(s)
}

/// Set given name (First name)
pub fn givenname(name: &str) -> PreambleElement {
    let s = texify!("givenname", name);
    PreambleElement::UserDefined(s)
}

/// Set family name (Last name)
pub fn familyname(name: &str) -> PreambleElement {
    let s = texify!("familyname", name);
    PreambleElement::UserDefined(s)
}

/// Set address
pub fn address(street: &str, city: Option<&str>, country: Option<&str>) -> PreambleElement {
    let mut s = texify!("address", street);
    match city {
        Some(city) => {
            s.push_str(&format!("{{{}}}", city));
        }
        None => {
            return PreambleElement::UserDefined(s);
        }
    }

    if let Some(country) = country {
        s.push_str(&format!("{{{}}}", country));
    }

    PreambleElement::UserDefined(s)
}

/// Set mobile number
pub fn mobile(number: &str) -> PreambleElement {
    let s = texify!("mobile", number);
    PreambleElement::UserDefined(s)
}

/// Set phone number
pub fn phone(number: &str, phone_type: Option<PhoneType>) -> PreambleElement {
    let s = texify!("phone", [phone_type], number);
    PreambleElement::UserDefined(s)
}

/// Set fax number
pub fn fax(number: &str) -> PreambleElement {
    let s = texify!("fax", number);
    PreambleElement::UserDefined(s)
}

/// Set email address
pub fn email(address: &str) -> PreambleElement {
    let s = texify!("email", address);
    PreambleElement::UserDefined(s)
}

/// Set social link
pub fn social(account: &str, social_type: SocialType, url: Option<&str>) -> PreambleElement {
    let mut s = texify!("social");
    s.push_str(&format!(r"[{}]", social_type));

    if let Some(url) = url {
        s.push_str(&format!(r"[{}]", url));
    }

    s.push_str(&format!(r"{{{}}}", account));

    PreambleElement::UserDefined(s)
}

/// Set home page
pub fn homepage(url: &str) -> PreambleElement {
    let s = texify!("homepage", url);
    PreambleElement::UserDefined(s)
}

/// Set moderncv theme
pub fn cvtheme(style: Style, color: Option<Color>) -> PreambleElement {
    let s = texify!("moderncvtheme", [color], style);
    PreambleElement::UserDefined(s)
}

/// Set extra information
pub fn extrainfo(info: &str) -> PreambleElement {
    let s = texify!("extrainfo", info);
    PreambleElement::UserDefined(s)
}

/// Set photo
pub fn photo(photo: &str, width: Option<&str>, frame: Option<&str>) -> PreambleElement {
    let s = texify!("photo", [width], [frame], photo);
    PreambleElement::UserDefined(s)
}

/// Set quote string
pub fn quote(msg: &str) -> PreambleElement {
    let s = texify!("quote", msg);
    PreambleElement::UserDefined(s)
}
