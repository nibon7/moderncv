//! An ergonomic library for programatically generating LaTeX resume.

use latex::DocumentClass;

pub mod preamble;
pub mod section;

pub use preamble::CVPreamble;
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
