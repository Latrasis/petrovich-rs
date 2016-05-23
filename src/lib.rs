//!
//! Petrovich is inflects Russian names to given grammatical case.
//! It supports first names, last names and middle names inflections.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/petrovich) and can be
//! used by adding `petrovich` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//!
//! petrovich = "0.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate petrovich;
//! ```
//!
//! # Examples
//!
//! ```
//! extern crate petrovich;
//!
//!
//! fn main() {
//!     let subject = petrovich::Petrovich::new();
//!     subject.firstname(Gender::Male, "Саша", Case::Dative).unwrap(); // Саше
//! }
//! ```

extern crate yaml_rust;

use std::io::prelude::*;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

/// Возможные Полы
#[derive(PartialEq, Clone, Copy)]
pub enum Gender {
    /// Мужской Пол
    Male,
    /// Женский Пол
    Female,
    /// Средний Пол
    Androgynous,
}

impl Gender {
    pub fn as_str(&self) -> &str {
        match *self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Androgynous => "androgynous",
        }
    }
}

/// Возможные Падежи
#[derive(PartialEq)]
pub enum Case {
    /// Родительный  | _Кого? Чего?_
    Genitive,
    /// Дательный    | _Кому? Чему?_
    Dative,
    /// Винительный  | _Кого? Что?_
    Accusative,
    /// Творительный | _Кем? Чем?_
    Instrumental,
    /// Предложный   | _О ком? О чём?_
    Prepositional,
}

impl Case {
    pub fn as_str(&self) -> &str {
        match *self {
            Case::Genitive => "genitive",
            Case::Dative => "dative",
            Case::Accusative => "accusative",
            Case::Instrumental => "instrumental",
            Case::Prepositional => "prepositional",
        }
    }
}

// Find Exception by name and gender
fn find_exception<'exc>(exceptions: &'exc Yaml, name: &str, gender: Gender) -> Option<&'exc Yaml> {

    // Search Exceptions with matching of name and gender
    exceptions.as_vec().and_then(|list| list.iter().find(|exception| {

        // Check if name matches
        let does_match_test = exception["test"]
                             .as_vec()
                             .unwrap()
                             .iter()
                             .any(|test| test.as_str().unwrap() == name.to_lowercase());
        // Check if gender matches
        let exception_gender = exception["gender"].as_str().unwrap();
        let does_match_gender = exception_gender == gender.as_str() || exception_gender == "androgynous";

        // Return true if both match
        does_match_test && does_match_gender
    }))
}

// Find Suffix by name and gender
fn find_suffix<'exc>(suffixes: &'exc Yaml, name: &str, gender: Gender) -> Option<&'exc Yaml> {

    let suffixes = suffixes.as_vec().unwrap();

    suffixes
        .iter()
        .filter(|suffix| {
            // Check if suffix matches
            let does_match_test = suffix["test"]
                                .as_vec()
                                .unwrap()
                                .iter()
                                .any(|test| name.to_lowercase().ends_with(test.as_str().unwrap()));

            // Check if gender matches
            let suffix_gender = suffix["gender"].as_str().unwrap();
            let does_match_gender = suffix_gender == gender.as_str() || suffix_gender == "androgynous";

            // Return true if both match
            does_match_test && does_match_gender
        })
        .max_by_key(|list| {
            // Find Longest Matching
            list["test"]
                .as_vec()
                .unwrap()
                .iter()
                .filter(|test| name.to_lowercase().ends_with(test.as_str().unwrap()))
                .max_by_key(|test| test.as_str().unwrap().len())
                .unwrap().as_str().unwrap().len()

        })
}

/// Initializes, Stores and applies Rules
pub struct Petrovich {
    firstname: Yaml,
    middlename: Yaml,
    lastname: Yaml,
}

impl Petrovich {
    /// Ru: Создавания нового Петровича
    /// En: Create a new inflector instance
    ///
    /// # Examples:
    ///
    ///  Simple inflection:
    ///
    /// ```
    /// let subject = Petrovich::new();
    /// ```
    ///
    pub fn new() -> Petrovich {
        use yaml_rust::yaml::Hash as YamlHash;

        // Open Rules File (Panics on error)
        let mut f = fs::File::open("./src/rules.yml").unwrap();
        // Create String Buffer and Read to it
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        // Pass Buffer to Yaml and unwrap
        let rules: &mut Yaml = &mut YamlLoader::load_from_str(&buffer).unwrap()[0];
        let rules: &mut YamlHash = match *rules {
            Yaml::Hash(ref mut hash) => hash,
            _ => panic!("not a hash"),
        };

        // Return Petrovich with preloaded rules
        Petrovich {
            firstname: rules.remove(&Yaml::String("firstname".into())).unwrap(),
            middlename: rules.remove(&Yaml::String("middlename".into())).unwrap(),
            lastname: rules.remove(&Yaml::String("lastname".into())).unwrap(),
        }
    }

    fn inflect(name: &str, rule: &Yaml, case: Case) -> String {

        // First unwrap to vector Vec<&str>
        let inflections = rule["mods"].as_vec().expect("No mods found");

        // Get inflection by Case
        let inflection = inflections[case as usize].as_str().unwrap_or("");

        // Count amount of dashes: "-" thus amount of characters left remaining
        let remaining: usize = name.chars().count() -
                               inflection.rfind("-").map_or(0, |pos| pos + 1);
        // Get Postfix
        let postfix = inflection.trim_left_matches("-");

        // Apply Inflection
        return name.chars().take(remaining).collect::<String>() + postfix;
    }

    pub fn firstname(&self, gender: Gender, name: &str, case: Case) -> Result<String, &str> {

        // First Let's Check for Exceptions
        find_exception(&self.firstname["exceptions"], name, gender)
        // Then Check for Suffixes
        .or(find_suffix(&self.firstname["suffixes"], name, gender))
            // If no matching found return error
            .ok_or("No Matching Firstname Rule Found")
            // Then Inflect Name using matched rule
            .and_then(|rule| Ok(Petrovich::inflect(name, rule, case)))
    }

    pub fn middlename(&self, gender: Gender, name: &str, case: Case) -> Result<String, &str> {

        // First Let's Check for Exceptions
        find_exception(&self.middlename["exceptions"], name, gender)
        // Then Check for Suffixes
        .or(find_suffix(&self.middlename["suffixes"], name, gender))
            // If no matching found return error
            .ok_or("No Matching Middlename Rule Found")
            // Then Inflect Name using matched rule
            .and_then(|rule| Ok(Petrovich::inflect(name, rule, case)))
    }

    pub fn lastname(&self, gender: Gender, name: &str, case: Case) -> Result<String, &str> {

        // First Let's Check for Exceptions
        find_exception(&self.lastname["exceptions"], name, gender)
        // Then Check for Suffixes
        .or(find_suffix(&self.lastname["suffixes"], name, gender))
            // If no matching found return error
            .ok_or("No Matching Lastname Rule Found")
            // Then Inflect Name using matched rule
            .and_then(|rule| Ok(Petrovich::inflect(name, rule, case)))
    }

    pub fn detect_gender(middlename: &str) -> Gender {
        if middlename.ends_with("ич") || middlename.ends_with("ыч") {
            return Gender::Male;
        }

        if middlename.ends_with("на") {
            return Gender::Female;
        }

        return Gender::Androgynous;
    }
}

#[test]
fn should_initialize() {
    let subject = Petrovich::new();
}

#[test]
fn should_error() {
    let subject = Petrovich::new();
    assert!(subject.firstname(Gender::Male, "Blabla", Case::Genitive).is_err());
    assert!(subject.middlename(Gender::Male, "Blabla", Case::Genitive).is_err());
    assert!(subject.lastname(Gender::Male, "Blabla", Case::Genitive).is_err());
}

#[test]
fn should_inflect_first_names() {
    let subject = Petrovich::new();

    assert_eq!("Лёшы",
               subject.firstname(Gender::Male, "Лёша", Case::Genitive).unwrap());
    assert_eq!("Лёше",
               subject.firstname(Gender::Male, "Лёша", Case::Dative).unwrap());
    assert_eq!("Лёшу",
               subject.firstname(Gender::Male, "Лёша", Case::Accusative).unwrap());
    assert_eq!("Лёшой",
               subject.firstname(Gender::Male, "Лёша", Case::Instrumental).unwrap());
    assert_eq!("Лёше",
               subject.firstname(Gender::Male, "Лёша", Case::Prepositional).unwrap());

    assert_eq!("Яши",
               subject.firstname(Gender::Male, "Яша", Case::Genitive).unwrap());
    assert_eq!("Яше",
               subject.firstname(Gender::Male, "Яша", Case::Dative).unwrap());
    assert_eq!("Яшу",
               subject.firstname(Gender::Male, "Яша", Case::Accusative).unwrap());
    assert_eq!("Яшей",
               subject.firstname(Gender::Male, "Яша", Case::Instrumental).unwrap());
    assert_eq!("Яше",
               subject.firstname(Gender::Male, "Яша", Case::Prepositional).unwrap());
}

#[test]
fn should_inflect_complex_male_lastnames() {
    let subject = Petrovich::new();

    assert_eq!("Бильжо",
        subject.lastname(Gender::Male, "Бильжо", Case::Dative).unwrap());
    assert_eq!("Ничипоруку",
        subject.lastname(Gender::Male, "Ничипорук", Case::Dative).unwrap());
    assert_eq!("Щусю",
        subject.lastname(Gender::Male, "Щусь", Case::Dative).unwrap());
    assert_eq!("Фидре",
        subject.lastname(Gender::Male, "Фидря", Case::Dative).unwrap());
    assert_eq!("Белоконю",
        subject.lastname(Gender::Male, "Белоконь", Case::Dative).unwrap());
    assert_eq!("Добробабе",
        subject.lastname(Gender::Male, "Добробаба", Case::Dative).unwrap());
    assert_eq!("Исайченко",
        subject.lastname(Gender::Male, "Исайченко", Case::Dative).unwrap());
    assert_eq!("Бондаришину",
        subject.lastname(Gender::Male, "Бондаришин", Case::Dative).unwrap());
    assert_eq!("Дубинке",
        subject.lastname(Gender::Male, "Дубинка", Case::Dative).unwrap());
    assert_eq!("Сироте",
        subject.lastname(Gender::Male, "Сирота", Case::Dative).unwrap());
    assert_eq!("Воеводе",
        subject.lastname(Gender::Male, "Воевода", Case::Dative).unwrap());
    assert_eq!("Воложу",
        subject.lastname(Gender::Male, "Волож", Case::Dative).unwrap());
    assert_eq!("Кравцу",
        subject.lastname(Gender::Male, "Кравец", Case::Dative).unwrap());
    assert_eq!("Самотечнему",
        subject.lastname(Gender::Male, "Самотечний", Case::Dative).unwrap());
    assert_eq!("Цою",
        subject.lastname(Gender::Male, "Цой", Case::Dative).unwrap());
    assert_eq!("Шопену",
        subject.lastname(Gender::Male, "Шопен", Case::Dative).unwrap());
    assert_eq!("Сосковцу",
        subject.lastname(Gender::Male, "Сосковец", Case::Dative).unwrap());

}

#[test]
fn should_inflect_complex_female_lastnames() {
    let subject = Petrovich::new();

    assert_eq!("Бильжо",
        subject.lastname(Gender::Female ,"Бильжо", Case::Dative).unwrap());
    assert_eq!("Ничипорук",
        subject.lastname(Gender::Female ,"Ничипорук", Case::Dative).unwrap());
    assert_eq!("Щусь",
        subject.lastname(Gender::Female ,"Щусь", Case::Dative).unwrap());
    assert_eq!("Фидре",
        subject.lastname(Gender::Female ,"Фидря", Case::Dative).unwrap());
    assert_eq!("Белоконь",
        subject.lastname(Gender::Female ,"Белоконь", Case::Dative).unwrap());
    assert_eq!("Добробабе",
        subject.lastname(Gender::Female ,"Добробаба", Case::Dative).unwrap());
    assert_eq!("Исайченко",
        subject.lastname(Gender::Female ,"Исайченко", Case::Dative).unwrap());
    assert_eq!("Бондаришин",
        subject.lastname(Gender::Female ,"Бондаришин", Case::Dative).unwrap());
    assert_eq!("Дубинке",
        subject.lastname(Gender::Female ,"Дубинка", Case::Dative).unwrap());
    assert_eq!("Сироте",
        subject.lastname(Gender::Female ,"Сирота", Case::Dative).unwrap());
    assert_eq!("Воеводе",
        subject.lastname(Gender::Female ,"Воевода", Case::Dative).unwrap());
    assert_eq!("Гулыге",
        subject.lastname(Gender::Female ,"Гулыга", Case::Dative).unwrap());
    assert_eq!("Дейнеке",
        subject.lastname(Gender::Female ,"Дейнека", Case::Dative).unwrap());
    assert_eq!("Джанджагава",
        subject.lastname(Gender::Female ,"Джанджагава", Case::Dative).unwrap());
    assert_eq!("Забейворота",
        subject.lastname(Gender::Female ,"Забейворота", Case::Dative).unwrap());
    assert_eq!("Окуджаве",
        subject.lastname(Gender::Female ,"Окуджава", Case::Dative).unwrap());
}

#[test]
fn should_detect_gender() {
    let subject = Petrovich::new();

    assert_eq!(Petrovich::detect_gender("Сергеевич").as_str(), Gender::Male.as_str());
    assert_eq!(Petrovich::detect_gender("Степаныч").as_str(), Gender::Male.as_str());
    assert_eq!(Petrovich::detect_gender("Петровна").as_str(), Gender::Female.as_str());
    assert_eq!(Petrovich::detect_gender("Оно").as_str(), Gender::Androgynous.as_str());
}
