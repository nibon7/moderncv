//! An ergonomic library for programatically generating LaTeX resume.

use latex::PreambleElement;

/// Define functions for moderncv
macro_rules! moderncv {
    ($fun:ident, $ret:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $fun<S: AsRef<str>>(s: S) -> $ret {
            <$ret>::UserDefined(format!(r"\{}{{{}}}", stringify!($fun), s.as_ref()))
        }
    };

    ($fun:ident, $opt:ident, $ret:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $fun<S: AsRef<str>>(s: S, opt: Option<S>) -> $ret {
            match opt {
                Some(opt) => <$ret>::UserDefined(format!(
                    r"\{}[{}]{{{}}}",
                    stringify!($fun),
                    opt.as_ref(),
                    s.as_ref()
                )),
                None => <$ret>::UserDefined(format!(r"\{}{{{}}}", stringify!($fun), s.as_ref(),)),
            }
        }
    };
}

moderncv!(firstname, PreambleElement, "First name");
moderncv!(familyname, PreambleElement, "Family name");
moderncv!(address, PreambleElement, "Address");
moderncv!(mobile, PreambleElement, "Mobile");
moderncv!(phone, PreambleElement, "Photo");
moderncv!(fax, PreambleElement, "Fax");
moderncv!(email, PreambleElement, "Email");
moderncv!(moderncvtheme, opt, PreambleElement, "Moderncv theme");
moderncv!(extrainfo, PreambleElement, "Extra infomation");
moderncv!(photo, opt, PreambleElement, "Photo");
moderncv!(quote, PreambleElement, "Quote");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_firstname() {
        assert_eq!(
            firstname("John"),
            PreambleElement::UserDefined(r"\firstname{John}".to_string())
        );
    }

    #[test]
    fn test_familyname() {
        assert_eq!(
            familyname("Doe"),
            PreambleElement::UserDefined(r"\familyname{Doe}".to_string())
        );
    }

    #[test]
    fn test_address() {
        assert_eq!(
            address("12 somestreet"),
            PreambleElement::UserDefined(r"\address{12 somestreet}".to_string())
        );
    }

    #[test]
    fn test_mobile() {
        assert_eq!(
            mobile("+123 456 7890"),
            PreambleElement::UserDefined(r"\mobile{+123 456 7890}".to_string())
        );
    }

    #[test]
    fn test_phone() {
        assert_eq!(
            phone("12 (3)456 78 90"),
            PreambleElement::UserDefined(r"\phone{12 (3)456 78 90}".to_string())
        );
    }

    #[test]
    fn test_fax() {
        assert_eq!(
            fax("12 (3)456 78 90"),
            PreambleElement::UserDefined(r"\fax{12 (3)456 78 90}".to_string())
        );
    }

    #[test]
    fn test_email() {
        assert_eq!(
            email("jdoe@design.org"),
            PreambleElement::UserDefined(r"\email{jdoe@design.org}".to_string())
        );
    }

    #[test]
    fn test_moderncvtheme() {
        assert_eq!(
            moderncvtheme("classic", None),
            PreambleElement::UserDefined(r"\moderncvtheme{classic}".to_string())
        );

        assert_eq!(
            moderncvtheme("casual", Some("green")),
            PreambleElement::UserDefined(r"\moderncvtheme[green]{casual}".to_string())
        );
    }

    #[test]
    fn test_extrainfo() {
        assert_eq!(
            extrainfo(r"\weblink{www.ctan.org}"),
            PreambleElement::UserDefined(r"\extrainfo{\weblink{www.ctan.org}}".to_string())
        );
    }

    #[test]
    fn test_photo() {
        assert_eq!(
            photo("jdoe_picture", Some("64pt")),
            PreambleElement::UserDefined(r"\photo[64pt]{jdoe_picture}".to_string())
        );
    }
    #[test]
    fn test_quote() {
        assert_eq!(
            quote("Any intelligent fool can make things bigger"),
            PreambleElement::UserDefined(
                r"\quote{Any intelligent fool can make things bigger}".to_string()
            )
        );
    }
}
