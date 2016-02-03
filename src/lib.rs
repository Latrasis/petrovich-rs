extern crate yaml_rust;

use std::io::prelude::*;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

// Initializes, Stores and applies Rules
struct Petrovich {
    middlename: Yaml,
    firstname: Yaml,
    lastname: Yaml
}

impl  Petrovich {

    fn new() -> Petrovich {

        // Open Rules File (Panics on error)
        let mut f = fs::File::open("./src/rules.yml").unwrap();
        // Create String Buffer and Read to it
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        // Pass Buffer to Yaml and unwrap
        let rules: &Yaml = &YamlLoader::load_from_str(&buffer).unwrap()[0];

        // Return Petrovich with preloaded rules
        Petrovich { 
            middlename: rules["middlename"],
            firstname: rules["firstname"],
            lastname: rules["lastname"]
        }
    }

    // TODO
    fn first_name(&self, gender: Gender, word: &str, case: Case) -> String {
        let ref firstname_rulesets = self.firstname;

        String::from(word)


    }
    // TODO
    fn last_name(&self, gender: Gender, word: &str, case: Case) -> String {
        String::from(word)
    }
    // TODO
    fn middle_name(&self, gender: Gender, word: &str, case: Case) -> String {
        String::from(word)
    }
}

// Predefined Genders
#[derive(PartialEq)]
enum Gender {
    Male,
    Female,
    Androgyenous,
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

#[test]
fn should_initialize() {
    let factory = Petrovich::new();
}

#[test]
fn should_parse_name() {
    let factory = Petrovich::new();

    assert_eq!("Лёша",
               factory.first_name(Gender::Male, "Лёша", Case::Genitive));
    assert_eq!("Лёше",
               factory.first_name(Gender::Male, "Лёша", Case::Dative));
    assert_eq!("Лёшу",
               factory.first_name(Gender::Male, "Лёша", Case::Accusative));
    assert_eq!("Лёшой",
               factory.first_name(Gender::Male, "Лёша", Case::Instrumental));
    assert_eq!("Лёше",
               factory.first_name(Gender::Male, "Лёша", Case::Prepositional));

}
