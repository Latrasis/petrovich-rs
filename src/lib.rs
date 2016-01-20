

struct Petrovich {
	gender: Gender
}

impl Petrovich {

	fn new(gender: Gender) -> Petrovich {
        Petrovich {
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

    let male = Petrovich::new(Gender::Male);
    assert!(male.gender == Gender::Female);
}

#[test]
fn it_works() {
	println!("{:?}", "Hello World!");
}
