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
    fn cventry(
        &mut self,
        years: &str,
        job: &str,
        employer: &str,
        localization: &str,
        grade: Option<&str>,
        comment: Option<&str>,
    ) -> &mut Self {
        let elem = self::cventry(years, job, employer, localization, grade, comment);
        self.push(elem);

        self
    }

    fn cvlanguage(&mut self, name: &str, level: &str, comment: &str) -> &mut Self {
        let elem = self::cvlanguage(name, level, comment);
        self.push(elem);

        self
    }

    fn cvcomputer(
        &mut self,
        category1: &str,
        programs1: &str,
        category2: &str,
        programs2: &str,
    ) -> &mut Self {
        let elem = self::cvcomputer(category1, programs1, category2, programs2);
        self.push(elem);

        self
    }

    fn cvline(&mut self, leftmark: &str, text: &str) -> &mut Self {
        let elem = self::cvline(leftmark, text);
        self.push(elem);

        self
    }

    fn cvdoubleitem(
        &mut self,
        subtitle1: &str,
        text1: &str,
        subtitle2: &str,
        text2: &str,
    ) -> &mut Self {
        let elem = self::cvdoubleitem(subtitle1, text1, subtitle2, text2);
        self.push(elem);

        self
    }

    fn cvlistitem(&mut self, item: &str) -> &mut Self {
        let elem = self::cvlistitem(item);
        self.push(elem);

        self
    }

    fn cvlistdoubleitem(&mut self, item1: &str, item2: &str) -> &mut Self {
        let elem = self::cvlistdoubleitem(item1, item2);
        self.push(elem);

        self
    }
}

/// Describe your education or your job experiences
pub fn cventry(
    years: &str,
    job: &str,
    employer: &str,
    localization: &str,
    grade: Option<&str>,
    comment: Option<&str>,
) -> Element {
    let mut s = texify!("cventry", years, job, employer, localization);

    match grade {
        Some(grade) => s.push_str(&format!(r"{{{}}}", grade)),
        None => s.push_str(r"{}"),
    }

    match comment {
        Some(comment) => s.push_str(&format!(r"{{{}}}", comment)),
        None => s.push_str(r"{}"),
    }

    Element::UserDefined(s)
}

/// Describe language skills
pub fn cvlanguage(name: &str, level: &str, comment: &str) -> Element {
    let s = texify!("cvlanguage", name, level, comment);
    Element::UserDefined(s)
}

/// Describe computer skills
pub fn cvcomputer(category1: &str, programs1: &str, category2: &str, programs2: &str) -> Element {
    let s = texify!("cvcomputer", category1, programs1, category2, programs2);
    Element::UserDefined(s)
}

/// Typeset lines with a hint on the left
pub fn cvline(leftmark: &str, text: &str) -> Element {
    let s = texify!("cvline", leftmark, text);
    Element::UserDefined(s)
}

/// Typeset entry with a description on the left, but in two columns inside a cvsection
pub fn cvdoubleitem(subtitle1: &str, text1: &str, subtitle2: &str, text2: &str) -> Element {
    let s = texify!("cvdoubleitem", subtitle1, text1, subtitle2, text2);
    Element::UserDefined(s)
}

/// Typeset lists on one column inside a cvsection
pub fn cvlistitem(item: &str) -> Element {
    let s = texify!("cvlistitem", item);
    Element::UserDefined(s)
}

/// Typeset lists on two columns inside a cvsection
pub fn cvlistdoubleitem(item1: &str, item2: &str) -> Element {
    let s = texify!("cvlistdoubleitem", item1, item2);
    Element::UserDefined(s)
}
