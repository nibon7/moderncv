use super::texify;
use latex::{Element, Section};

/// A trait which implement useful functions for moderncv section
pub trait CVSection {
    fn cventry(
        &mut self,
        year: &str,
        job: &str,
        institution: &str,
        localization: &str,
        grade: Option<&str>,
        comment: Option<&str>,
    ) -> &mut Self;
    fn cvlanguage(&mut self, name: &str, level: &str, comment: &str) -> &mut Self;
    fn cvcomputer(
        &mut self,
        category: &str,
        programs: &str,
        category: &str,
        programs: &str,
    ) -> &mut Self;
    fn cvline(&mut self, leftmark: &str, text: &str) -> &mut Self;
    fn cvdoubleitem(
        &mut self,
        subtitle1: &str,
        text1: &str,
        subtitle2: &str,
        text2: &str,
    ) -> &mut Self;
    fn cvlistitem(&mut self, item: &str) -> &mut Self;
    fn cvlistdoubleitem(&mut self, item1: &str, item2: &str) -> &mut Self;
}

impl CVSection for Section {
    /// Describe your education or your job experiences
    fn cventry(
        &mut self,
        years: &str,
        job: &str,
        employer: &str,
        localization: &str,
        grade: Option<&str>,
        comment: Option<&str>,
    ) -> &mut Self {
        let mut s = texify!("cventry", years, job, employer, localization);

        match grade {
            Some(grade) => s.push_str(&format!(r"{{{}}}", grade)),
            None => s.push_str(r"{}"),
        }

        match comment {
            Some(comment) => s.push_str(&format!(r"{{{}}}", comment)),
            None => s.push_str(r"{}"),
        }

        let elem = Element::UserDefined(s);
        self.push(elem);

        self
    }

    /// Describe language skills
    fn cvlanguage(&mut self, name: &str, level: &str, comment: &str) -> &mut Self {
        let s = texify!("cvlanguage", name, level, comment);
        let elem = Element::UserDefined(s);
        self.push(elem);

        self
    }

    /// Describe computer skills
    fn cvcomputer(
        &mut self,
        category1: &str,
        programs1: &str,
        category2: &str,
        programs2: &str,
    ) -> &mut Self {
        let s = texify!("cvcomputer", category1, programs1, category2, programs2);
        let elem = Element::UserDefined(s);
        self.push(elem);

        self
    }

    /// Typeset lines with a hint on the left
    fn cvline(&mut self, leftmark: &str, text: &str) -> &mut Self {
        let s = texify!("cvline", leftmark, text);
        let elem = Element::UserDefined(s);
        self.push(elem);

        self
    }

    /// Typeset entry with a description on the left, but in two columns inside a cvsection
    fn cvdoubleitem(
        &mut self,
        subtitle1: &str,
        text1: &str,
        subtitle2: &str,
        text2: &str,
    ) -> &mut Self {
        let s = texify!("cvdoubleitem", subtitle1, text1, subtitle2, text2);
        let elem = Element::UserDefined(s);
        self.push(elem);

        self
    }

    /// Typeset lists on one column inside a cvsection
    fn cvlistitem(&mut self, item: &str) -> &mut Self {
        let s = texify!("cvlistitem", item);
        let elem = Element::UserDefined(s);
        self.push(elem);

        self
    }

    /// Typeseet lists on two columns inside a cvsection
    fn cvlistdoubleitem(&mut self, item1: &str, item2: &str) -> &mut Self {
        let s = texify!("cvlistdoubleitem", item1, item2);
        let elem = Element::UserDefined(s);
        self.push(elem);

        self
    }
}
