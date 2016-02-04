extern crate yaml_rust;

use std::io::prelude::*;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

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

// Initializes, Stores and applies Rules
struct Petrovich {
    firstname: Yaml,
    middlename: Yaml,
    lastname: Yaml
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

    // TODO
    fn first_name(&self, gender: Gender, name: &str, case: Case) -> String {

        // First Let's Check for Exceptions
        let exceptions = self.firstname["exceptions"].as_vec().unwrap();
        for exception in exceptions {
            if exception["test"].as_str().unwrap() == name {
                println!("{:?}", name);
            }
        }
        // If No Exceptions Matched we Check for Suffixes

        // Once the correct rule is found we apply the rule

        String::from(name)
    }

    // TODO
    fn middle_name(&self, gender: Gender, name: &str, case: Case) -> String {
        String::from(name)
    }

    // TODO
    fn last_name(&self, gender: Gender, name: &str, case: Case) -> String {
        String::from(name)
    }
}

#[test]
fn should_initialize() {
    let factory = Petrovich::new();
}

#[test]
fn should_inflect_first_name() {
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
