extern crate yaml_rust;

use std::io::prelude::*;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

// Initializes, Stores and applies Rules
struct Petrovich {
    rules: Vec<Yaml>,
}

impl Petrovich {

    fn new() -> Petrovich {

        // Open Rules File (Panics on error)
        let mut f = fs::File::open("./src/rules.yml").unwrap();
        // Create String Buffer and Read to it
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        // Pass Buffer to Yaml and unwrap
        let data = YamlLoader::load_from_str(&buffer).unwrap();

        // Return Petrovich with preloaded rules
        Petrovich { rules: data }
    }

    // TODO
    fn last_name(&self, person: &Person, word: &str, case: Case) -> String {
        String::from(word)
    }
    // TODO
    fn first_name(&self, person: &Person, word: &str, case: Case) -> String {
        String::from(word)
    }
    // TODO
    fn middle_name(&self, person: &Person, word: &str, case: Case) -> String {
        String::from(word)
    }
}

struct Person {
    gender: Gender,
}

impl Person {
    fn new(gender: Gender) -> Person {
        Person { gender: gender }
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
fn should_create_person() {
    let male = Person::new(Gender::Male);
    assert!(male.gender == Gender::Male);
}

#[test]
fn should_case_name() {
    let factory = Petrovich::new();
    let male = &Person::new(Gender::Male);

    assert_eq!("Лёша",
               factory.first_name(male, "Лёша", Case::Genitive));
    assert_eq!("Лёше",
               factory.first_name(male, "Лёша", Case::Dative));
    assert_eq!("Лёшу",
               factory.first_name(male, "Лёша", Case::Accusative));
    assert_eq!("Лёшой",
               factory.first_name(male, "Лёша", Case::Instrumental));
    assert_eq!("Лёше",
               factory.first_name(male, "Лёша", Case::Prepositional));

}
