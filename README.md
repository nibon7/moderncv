# moderncv-rs

An ergonomic library for programatically generating LaTeX resume.

This crate implements most of the useful functions for [moderncv](https://github.com/moderncv/moderncv).
It is still in early stage and the API may change.


## Getting started

* Create a moderncv document.

  Use `moderncv::document_class()` to specify the type of the document.

* Write a resume with rust

  Import `moderncv::{CVPreamble, CVSection}` and then you can use functions
  provided by moderncv with `latex::Preamble` and `latex::Section`.

* Generating pdf

  Use `pdflatex` or even `lualatex` to generate pdf file.


## Examples

```rust
use latex::{Document, Element, Section};
use moderncv::{document_class, CVPreamble, CVSection, Color, PhoneType, SocialType, Style};

fn main() {
    // Create LaTeX document
    let mut doc = Document::new(document_class());

    // Personal information
    doc.preamble
        .title("My Resume")
        .cvtheme(Style::Classic, Some(Color::Green))
        .firstname("Firstname")
        .lastname("Lastname")
        .photo("myphoto.jpg", Some("64pt"), None)
        .address("Street", Some("Your city"), Some("Your country"))
        .phone("+1234567890", Some(PhoneType::Mobile))
        .email("address@domain.com")
        .social("github_account", SocialType::Github, None)
        .homepage("https://xxx.yyy.zzz/page");

    doc.push(Element::TitlePage);

    // Education details
    let mut education = Section::new("Education");
    education
        .cventry(
            "year--year",
            "Degree",
            "Institution",
            "City",
            Some("Grade"),
            Some("Description"),
        )
        .cventry(
            "year--year",
            "Degree",
            "Institution",
            "City",
            Some("Grade"),
            Some("Description"),
        );
    doc.push(education);

    let mut thesis = Section::new("Master thesis");
    thesis
        .cvline("title", r"\emph{Title}")
        .cvline("supervisor", "Supervisors")
        .cvline("descriptin", r"\small Short thesis abstract");
    doc.push(thesis);

    // Experience
    let mut experience = Section::new("Experience");
    experience
        .cventry(
            "year--year",
            "Job title 1",
            "Employer 1",
            "City 1",
            None,
            Some("Description 1"),
        )
        .cventry(
            "year--year",
            "Job title 2",
            "Employer 2",
            "City 2",
            None,
            Some("Description"),
        );
    doc.push(experience);

    // Language skills
    let mut languages = Section::new("Languages");
    languages
        .cvlanguage("Language 1", "Skill level", "Comment")
        .cvlanguage("Language 2", "Skill level", "Comment");
    doc.push(languages);

    // Computer skills
    let mut computer = Section::new("Computer skills");
    computer
        .cvcomputer("Category 1", "XXX, YYY, ZZZ", "Category 3", "XXX, YYY, ZZZ")
        .cvcomputer("Category 2", "XXX, YYY, ZZZ", "Category 4", "XXX, YYY, ZZZ");
    doc.push(computer);

    // Interests
    let mut interests = Section::new("Interests");
    interests
        .cvline("Hobby 1", r"\small Description")
        .cvline("Hobby 2", r"\small Description")
        .cvline("Hobby 3", r"\small Description");
    doc.push(interests);

    // Extra
    let mut extra = Section::new("Extra");
    extra
        .cvlistitem("Item 1")
        .cvlistitem("Item 2")
        .cvlistitem("Item 3");
    doc.push(extra);

    let rendered = latex::print(&doc).unwrap();

    println!("{}", rendered);
}
```
