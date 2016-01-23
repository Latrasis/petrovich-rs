
struct Person {
	gender: Gender
}

impl Person {

	fn new(gender: Gender) -> Person {
        Person {
			gender: gender
		}
	}

	// fn last_name(self, word: &str, case: Case) -> String {}
	// fn first_name(self, word: &str, case: Case) -> String {}
	// fn middle_name(self, word: &str,case: Case) -> String {}
}

// Predefined Genders

#[derive(PartialEq)]
enum Gender {
	Male,
	Female,
	Androgyenous
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
fn should_create_person(){

    let male = Person::new(Gender::Male);
    assert!(male.gender == Gender::Male);
}
