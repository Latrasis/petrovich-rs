

struct Person {
	gender: Gender
}

impl Person {

	fn new(gender: Gender) -> Person {
        Person {
			gender: gender
		}
	}
}

// Predefined Genders

#[derive(PartialEq)]
enum Gender {
	Male,
	Female,
	Androgyenous
}

#[test]
fn example_test(){

    let male = Person::new(Gender::Male);
    assert!(male.gender == Gender::Female);
}

#[test]
fn it_works() {
	println!("{:?}", "Hello World!");
}
