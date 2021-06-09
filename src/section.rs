use super::texify;
use latex::{Element, Section};

/// A trait which implement useful functions for moderncv section
pub trait CVSection {
    fn cventry(
        &mut self,
        year: &str,
        job: &str,
        employer: &str,
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
    fn cvitem(&mut self, header: &str, text: &str) -> &mut Self;
    fn cvdoubleitem(&mut self, header1: &str, text1: &str, header2: &str, text2: &str)
        -> &mut Self;
    fn cvlistitem(&mut self, item: &str) -> &mut Self;
    fn cvlistdoubleitem(&mut self, item1: &str, item2: &str) -> &mut Self;
    fn cvitemwithcomment(&mut self, header: &str, text: &str, comment: &str) -> &mut Self;
}

impl CVSection for Section {
    /// Make a typical resume job / education entry
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

    /// Make a resume entry to describe language skills
    fn cvlanguage(&mut self, name: &str, level: &str, comment: &str) -> &mut Self {
        let elem = self::cvlanguage(name, level, comment);
        self.push(elem);

        self
    }

    /// Make a resume entry to describe computer skills
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

    /// Make a resume line with a header and a corresponding text (Alias of `cvitem`)
    fn cvline(&mut self, leftmark: &str, text: &str) -> &mut Self {
        let elem = self::cvline(leftmark, text);
        self.push(elem);

        self
    }

    /// Make a resume line with a header and a corresponding text
    fn cvitem(&mut self, header: &str, text: &str) -> &mut Self {
        let elem = self::cvitem(header, text);
        self.push(elem);

        self
    }

    /// Make a resume line with two headers and their corresponding text
    fn cvdoubleitem(
        &mut self,
        header1: &str,
        text1: &str,
        header2: &str,
        text2: &str,
    ) -> &mut Self {
        let elem = self::cvdoubleitem(header1, text1, header2, text2);
        self.push(elem);

        self
    }

    /// Make a resume line with a list item
    fn cvlistitem(&mut self, item: &str) -> &mut Self {
        let elem = self::cvlistitem(item);
        self.push(elem);

        self
    }

    /// Make a resume line with two list items
    fn cvlistdoubleitem(&mut self, item1: &str, item2: &str) -> &mut Self {
        let elem = self::cvlistdoubleitem(item1, item2);
        self.push(elem);

        self
    }

    /// Make a resume entry with a proficiency comment
    fn cvitemwithcomment(&mut self, header: &str, text: &str, comment: &str) -> &mut Self {
        let elem = self::cvitemwithcomment(header, text, comment);
        self.push(elem);

        self
    }
}

/// Make a typical resume job / education entry
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

/// Make a resume entry to describe language skills
pub fn cvlanguage(name: &str, level: &str, comment: &str) -> Element {
    let s = texify!("cvlanguage", name, level, comment);
    Element::UserDefined(s)
}

/// Make a resume entry to describe computer skills
pub fn cvcomputer(category1: &str, programs1: &str, category2: &str, programs2: &str) -> Element {
    let s = texify!("cvcomputer", category1, programs1, category2, programs2);
    Element::UserDefined(s)
}

/// Make a resume line with a header and a corresponding text (Alias of `cvitem`)
pub fn cvline(header: &str, text: &str) -> Element {
    let s = texify!("cvline", header, text);
    Element::UserDefined(s)
}

/// Make a resume line with a header and a corresponding text
pub fn cvitem(header: &str, text: &str) -> Element {
    let s = texify!("cvitem", header, text);
    Element::UserDefined(s)
}

/// Make a resume line with two headers and their corresponding text
pub fn cvdoubleitem(header1: &str, text1: &str, header2: &str, text2: &str) -> Element {
    let s = texify!("cvdoubleitem", header1, text1, header2, text2);
    Element::UserDefined(s)
}

/// Make a resume line with a list item
pub fn cvlistitem(item: &str) -> Element {
    let s = texify!("cvlistitem", item);
    Element::UserDefined(s)
}

/// Make a resume line with two list items
pub fn cvlistdoubleitem(item1: &str, item2: &str) -> Element {
    let s = texify!("cvlistdoubleitem", item1, item2);
    Element::UserDefined(s)
}

/// Make a resume entry with a proficiency comment
pub fn cvitemwithcomment(header: &str, text: &str, comment: &str) -> Element {
    let s = texify!("cvitemwithcomment", header, text, comment);
    Element::UserDefined(s)
}
