extern crate yaml_rust;

use std::io::prelude::*;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

// Predefined Genders
#[derive(PartialEq, Clone, Copy)]
enum Gender {
    Male,
    Female,
    Androgynous,
}

impl Gender {
    fn as_str(&self) -> &str {
        match *self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Androgynous => "androgynous",
        }
    }
}

// Predefined Cases
#[derive(PartialEq)]
enum Case {
    Genitive,
    Dative,
    Accusative,
    Instrumental,
    Prepositional,
}

impl Case {
    fn as_str(&self) -> &str {
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

    // First Let's Check for Exceptions
    let exceptions = exceptions.as_vec().unwrap();

    // Search Exceptions with matching of name and gender
    exceptions.iter().find(|exception| {

        // Check if name matches
        let match_test = exception["test"]
                             .as_vec()
                             .unwrap()
                             .iter()
                             .any(|test| test.as_str().unwrap() == name.to_lowercase());
        // Check if gender matches
        let match_gender = exception["gender"].as_str().unwrap() == gender.as_str();
        // Return true if both match
        match_test && match_gender
    })
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

// Initializes, Stores and applies Rules
struct Petrovich {
    firstname: Yaml,
    middlename: Yaml,
    lastname: Yaml,
}

impl Petrovich {

    fn new() -> Petrovich {
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
        let inflections = rule["mods"].as_vec().unwrap();

        // Get inflection by Case
        let inflection = inflections[case as usize].as_str().unwrap();

        // Count amount of dashes: "-" thus amount of characters left remaining
        let remaining: usize = name.chars().count() -
                               inflection.rfind("-").map_or(0, |pos| pos + 1);
        // Get Postfix
        let postfix = inflection.trim_left_matches("-");

        // Apply Inflection
        return name.chars().take(remaining).collect::<String>() + postfix;
    }

    // TODO
    fn first_name(&self, gender: Gender, name: &str, case: Case) -> Result<String, &str> {

        // First Let's Check for Exceptions
        find_exception(&self.firstname["exceptions"], name, gender)
        // Then Check for Suffixes
        .or(find_suffix(&self.firstname["suffixes"], name, gender))
            // If no matching found return error
            .ok_or("No Matching Rule Found")
            // Then Inflect Name using matched rule
            .and_then(|rule| Ok(Petrovich::inflect(name, rule, case)))
    }

    // TODO
    fn middle_name(&self, gender: Gender, name: &str, case: Case) -> String {
        String::from("")
    }

    // TODO
    fn last_name(&self, gender: Gender, name: &str, case: Case) -> String {
        String::from("")
    }
}

#[test]
fn should_initialize() {
    let factory = Petrovich::new();
}

#[test]
fn should_inflect_first_name() {
    let factory = Petrovich::new();

    // // Лёша
    assert_eq!("Лёшы",
               factory.first_name(Gender::Male, "Лёша", Case::Genitive).unwrap());
    assert_eq!("Лёше",
               factory.first_name(Gender::Male, "Лёша", Case::Dative).unwrap());
    assert_eq!("Лёшу",
               factory.first_name(Gender::Male, "Лёша", Case::Accusative).unwrap());
    assert_eq!("Лёшой",
               factory.first_name(Gender::Male, "Лёша", Case::Instrumental).unwrap());
    assert_eq!("Лёше",
               factory.first_name(Gender::Male, "Лёша", Case::Prepositional).unwrap());

    assert_eq!("Яши",
               factory.first_name(Gender::Male, "Яша", Case::Genitive).unwrap());
    assert_eq!("Яше",
               factory.first_name(Gender::Male, "Яша", Case::Dative).unwrap());
    assert_eq!("Яшу",
               factory.first_name(Gender::Male, "Яша", Case::Accusative).unwrap());
    assert_eq!("Яшей",
               factory.first_name(Gender::Male, "Яша", Case::Instrumental).unwrap());
    assert_eq!("Яше",
               factory.first_name(Gender::Male, "Яша", Case::Prepositional).unwrap());

}
