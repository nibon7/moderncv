//! An ergonomic library for programatically generating LaTeX resume.

use latex::PreambleElement;

macro_rules! moderncv {
    ($fun:ident, {$arg:ident}, $ret:ty, $doc:literal) => {
        #[doc = $doc]
        pub fn $fun<S: AsRef<str>>($arg: S) -> $ret {
            <$ret>::UserDefined(format!(r"\{}{{{}}}", stringify!($fun), $arg.as_ref()))
        }
    };

    ($fun:ident, {$arg:ident}, {$extra:ident}, $ret:ty, $doc:literal) => {
        #[doc = $doc]
        pub fn $fun<S: AsRef<str>>($arg: S, $extra: S) -> $ret {
            <$ret>::UserDefined(format!(
                r"\{}{{{}}}{{{}}}",
                stringify!($fun),
                $arg.as_ref(),
                $extra.as_ref()
            ))
        }
    };

    ($fun:ident, {$arg:ident}, {$extra1:ident}, {$extra2:ident}, $ret:ty, $doc:literal) => {
        #[doc = $doc]
        pub fn $fun<S: AsRef<str>>($arg: S, $extra1: Option<S>, $extra2: Option<S>) -> $ret {
            let mut s = format!(r"\{}{{{}}}", stringify!($fun), $arg.as_ref());
            match $extra1 {
                Some(extra) => s.push_str(&format!(r"{{{}}}", extra.as_ref())),
                None => return <$ret>::UserDefined(s),
            }

            if let Some(extra) = $extra2 {
                s.push_str(&format!(r"{{{}}}", extra.as_ref()));
            }

            <$ret>::UserDefined(s)
        }
    };

    ($fun:ident, [$opt:ident], {$arg:ident}, $ret:ty, $doc:literal) => {
        #[doc = $doc]
        pub fn $fun<S: AsRef<str>>($arg: S, $opt: Option<S>) -> $ret {
            let s = match $opt {
                Some(opt) => format!(
                    r"\{}[{}]{{{}}}",
                    stringify!($fun),
                    opt.as_ref(),
                    $arg.as_ref()
                ),
                None => format!(r"\{}{{{}}}", stringify!($fun), $arg.as_ref()),
            };

            <$ret>::UserDefined(s)
        }
    };
}

moderncv!(name, { fistname }, { lastname }, PreambleElement, "Name");
moderncv!(firstname, { firstname }, PreambleElement, "First name");
moderncv!(familyname, { familyname }, PreambleElement, "Family name");
moderncv!(
    address,
    { street },
    { city },
    { country },
    PreambleElement,
    "Address"
);
moderncv!(mobile, { mobile }, PreambleElement, "Mobile");
moderncv!(phone, { phone }, PreambleElement, "Phone");
moderncv!(fax, { fax }, PreambleElement, "Fax");
moderncv!(email, { email }, PreambleElement, "Email");
moderncv!(
    moderncvtheme,
    [opt],
    { theme },
    PreambleElement,
    "Moderncv theme"
);
moderncv!(
    extrainfo,
    { extrainfo },
    PreambleElement,
    "Extra infomation"
);
moderncv!(photo, [opt], { photo }, PreambleElement, "Photo");
moderncv!(quote, { quote }, PreambleElement, "Quote");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            name("John", "Doe"),
            PreambleElement::UserDefined(r"\name{John}{Doe}".to_string())
        );
    }

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
            address("12 somestreet", Some("3456 somecity"), None),
            PreambleElement::UserDefined(r"\address{12 somestreet}{3456 somecity}".to_string())
        );

        assert_eq!(
            address("12 somestreet", None, Some("unused")),
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
