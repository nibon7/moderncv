//! An ergonomic library for programatically generating LaTeX resume.

use latex::{DocumentClass, Element};

pub mod preamble;
pub mod section;

pub use preamble::{CVPreamble, Color, PhoneType, SocialType, Style};
pub use section::CVSection;

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

/// Close section(`\closesection{}`)
pub fn closesection() -> Element {
    let empty = "";
    let s = texify!("closesection", empty);

    Element::UserDefined(s)
}

/// Empty section(`\emptysection{}`)
pub fn emptysection() -> Element {
    let empty = "";
    let s = texify!("emptysection", empty);

    Element::UserDefined(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_class() {
        let s = document_class().to_string();
        assert!(s.eq("moderncv"));
    }

    #[test]
    fn test_closesection() {
        let left = closesection();
        let right = Element::UserDefined(r"\closesection{}".to_string());

        assert!(left.eq(&right));
    }

    #[test]
    fn test_emptysection() {
        let left = emptysection();
        let right = Element::UserDefined(r"\emptysection{}".to_string());

        assert!(left.eq(&right));
    }
}
