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
//! petrovich = "0.2"
//! ```
//!
//! # Examples
//!
//! ```
//!
//! use petrovich::*;
//!
//! fn main() {
//!     assert_eq!(firstname(Gender::Male, "Саша", Case::Dative), "Саше");
//!     assert_eq!(firstname(Gender::Female, "Изабель", Case::Genitive), "Изабель");
//!
//!     assert_eq!(lastname(Gender::Male, "Станкевич", Case::Prepositional), "Станкевиче");
//!     assert_eq!(lastname(Gender::Female, "Станкевич", Case::Prepositional), "Станкевич");
//!
//!     assert_eq!(middlename(Gender::Male, "Сергеич", Case::Instrumental), "Сергеичем");
//!     assert_eq!(middlename(Gender::Female, "Прокопьевна", Case::Accusative), "Прокопьевну");
//! }
//! ```

mod gender;
pub use gender::{detect_gender, Gender};

pub mod deprecated;
pub use deprecated::*;

type Modifier = Option<(usize, &'static str)>;

#[derive(Eq, PartialEq, Copy, Clone)]
enum RuleTag {
    FirstWord,
}

use RuleTag::*;

struct Rule {
    gender: Gender,
    test: &'static [&'static str],
    mods: [Modifier; 5],
    tags: &'static [RuleTag],
}

impl Rule {
    fn modifier(&self, case: Case) -> Modifier {
        self.mods[case as usize]
    }

    fn has_tag(&self, tag: RuleTag) -> bool {
        self.tags.contains(&tag)
    }

    fn fully_matches(&self, name: &str) -> bool {
        self.test.iter().any(|&test| test == name)
    }

    fn suffix_matches(&self, name: &str) -> bool {
        self.test.iter().any(|&test| name.ends_with(test))
    }

    fn gender_matches(&self, gender: Gender) -> bool {
        self.gender == gender || self.gender == Gender::Androgynous
    }
}

struct RuleList {
    exceptions: &'static [Rule],
    suffixes: &'static [Rule],
}

struct Rules {
    lastname: RuleList,
    firstname: RuleList,
    middlename: RuleList,
}

const RULES: Rules = include!(concat!(env!("OUT_DIR"), "/rules.inc"));

/// Возможные падежи
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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

// Find exception by name and gender
fn find_exception<'a>(
    exceptions: &'a [Rule],
    name: &str,
    gender: Gender,
    is_last: bool,
) -> Option<&'a Rule> {
    // Search exceptions with matching name and gender
    exceptions.iter().find(|&exception| {
        exception.fully_matches(name)
            && exception.gender_matches(gender)
            && (!exception.has_tag(FirstWord) || !is_last)
    })
}

// Find suffix by name and gender
fn find_suffix<'a>(suffixes: &'a [Rule], name: &str, gender: Gender) -> Option<&'a Rule> {
    suffixes
        .iter()
        .filter(|&suffix| suffix.suffix_matches(name) && suffix.gender_matches(gender))
        .max_by_key(|&rule| {
            // Find longest match
            rule.test
                .iter()
                .filter(|&&test| name.ends_with(test))
                .max_by_key(|&&test| test.len())
                .unwrap()
                .len()
        })
}

fn inflect(name: &str, rule: &Rule, case: Case) -> String {
    // Get inflection by case
    if let Some((skip, postfix)) = rule.modifier(case) {
        name.chars()
            .take(name.chars().count() - skip)
            .collect::<String>()
            + postfix
    } else {
        name.to_owned()
    }
}

fn inflect_name_part(
    gender: Gender,
    name: &str,
    case: Case,
    rule_list: &RuleList,
    is_last: bool,
) -> Option<String> {
    let lowercase_name = name.to_lowercase();
    // First let's check for exceptions
    find_exception(rule_list.exceptions, &lowercase_name, gender, is_last)
        // Then check for suffixes
        .or(find_suffix(rule_list.suffixes, &lowercase_name, gender))
        // Then inflect name using matched rule
        .map(|rule| inflect(name, rule, case))
}

fn inflect_name(gender: Gender, name: &str, case: Case, rule_list: &RuleList) -> String {
    let name_parts: Vec<&str> = name.split('-').collect();
    name_parts
        .iter()
        .enumerate()
        .map(|(i, &name_part)| {
            inflect_name_part(
                gender,
                name_part,
                case,
                rule_list,
                i == name_parts.len() - 1,
            )
            .unwrap_or(name_part.to_owned())
        })
        .collect::<Vec<_>>()
        .join("-")
}

/// Inflects first name
pub fn firstname(gender: Gender, name: &str, case: Case) -> String {
    inflect_name(gender, name, case, &RULES.firstname)
}

/// Inflects last name
pub fn lastname(gender: Gender, name: &str, case: Case) -> String {
    inflect_name(gender, name, case, &RULES.lastname)
}

/// Inflects middle name
pub fn middlename(gender: Gender, name: &str, case: Case) -> String {
    inflect_name(gender, name, case, &RULES.middlename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_through_uninflected() {
        assert_eq!(firstname(Gender::Male, "Blabla", Case::Genitive), "Blabla");
        assert_eq!(middlename(Gender::Male, "Blabla", Case::Genitive), "Blabla");
        assert_eq!(lastname(Gender::Male, "Blabla", Case::Genitive), "Blabla");
    }

    #[test]
    fn should_inflect_first_names() {
        assert_eq!(firstname(Gender::Male, "Лёша", Case::Genitive), "Лёши");
        assert_eq!(firstname(Gender::Male, "Лёша", Case::Dative), "Лёше");
        assert_eq!(firstname(Gender::Male, "Лёша", Case::Accusative), "Лёшу");
        assert_eq!(firstname(Gender::Male, "Лёша", Case::Instrumental), "Лёшей");
        assert_eq!(firstname(Gender::Male, "Лёша", Case::Prepositional), "Лёше");

        assert_eq!(firstname(Gender::Male, "Яша", Case::Genitive), "Яши");
        assert_eq!(firstname(Gender::Male, "Яша", Case::Dative), "Яше");
        assert_eq!(firstname(Gender::Male, "Яша", Case::Accusative), "Яшу");
        assert_eq!(firstname(Gender::Male, "Яша", Case::Instrumental), "Яшей");
        assert_eq!(firstname(Gender::Male, "Яша", Case::Prepositional), "Яше");
        assert_eq!(
            firstname(Gender::Male, "Илья-Александр", Case::Dative),
            "Илье-Александру"
        );
    }

    #[test]
    fn should_inflect_complex_male_lastnames() {
        assert_eq!(lastname(Gender::Male, "Кваша", Case::Genitive), "Кваши");
        assert_eq!(lastname(Gender::Male, "Бильжо", Case::Dative), "Бильжо");
        assert_eq!(
            lastname(Gender::Male, "Ничипорук", Case::Dative),
            "Ничипоруку"
        );
        assert_eq!(lastname(Gender::Male, "Щусь", Case::Dative), "Щусю");
        assert_eq!(lastname(Gender::Male, "Фидря", Case::Dative), "Фидре");
        assert_eq!(lastname(Gender::Male, "Белоконь", Case::Dative), "Белоконю");
        assert_eq!(
            lastname(Gender::Male, "Добробаба", Case::Dative),
            "Добробабе"
        );
        assert_eq!(
            lastname(Gender::Male, "Исайченко", Case::Dative),
            "Исайченко"
        );
        assert_eq!(
            lastname(Gender::Male, "Бондаришин", Case::Dative),
            "Бондаришину"
        );
        assert_eq!(lastname(Gender::Male, "Дубинка", Case::Dative), "Дубинке");
        assert_eq!(lastname(Gender::Male, "Сирота", Case::Dative), "Сироте");
        assert_eq!(lastname(Gender::Male, "Воевода", Case::Dative), "Воеводе");
        assert_eq!(lastname(Gender::Male, "Волож", Case::Dative), "Воложу");
        assert_eq!(lastname(Gender::Male, "Кравец", Case::Dative), "Кравцу");
        assert_eq!(
            lastname(Gender::Male, "Самотечний", Case::Dative),
            "Самотечнему",
        );
        assert_eq!(lastname(Gender::Male, "Цой", Case::Dative), "Цою");
        assert_eq!(lastname(Gender::Male, "Вий", Case::Dative), "Вию");
        assert_eq!(lastname(Gender::Male, "Шопен", Case::Dative), "Шопену");
        assert_eq!(lastname(Gender::Male, "Сосковец", Case::Dative), "Сосковцу");
        assert_eq!(
            lastname(Gender::Male, "Иванов-Сидоров", Case::Dative),
            "Иванову-Сидорову"
        );
        assert_eq!(
            lastname(Gender::Male, "Петров Водкин", Case::Dative),
            "Петров Водкину"
        );
        assert_eq!(lastname(Gender::Male, "Бонч", Case::Dative), "Бончу");
        assert_eq!(
            lastname(Gender::Male, "Бонч-Бруевич", Case::Dative),
            "Бонч-Бруевичу"
        );
    }

    #[test]
    fn should_inflect_middlenames() {
        assert_eq!(middlename(Gender::Male, "фон", Case::Genitive), "фон");
        assert_eq!(middlename(Gender::Female, "Борух", Case::Dative), "Борух");
        assert_eq!(
            middlename(Gender::Female, "Борух-Бендитовна", Case::Dative),
            "Борух-Бендитовне"
        );
        assert_eq!(
            middlename(Gender::Female, "Георгиевна-Авраамовна", Case::Dative),
            "Георгиевне-Авраамовне"
        )
    }

    #[test]
    fn should_inflect_complex_female_lastnames() {
        assert_eq!(lastname(Gender::Female, "Бильжо", Case::Dative), "Бильжо");
        assert_eq!(
            lastname(Gender::Female, "Ничипорук", Case::Dative),
            "Ничипорук"
        );
        assert_eq!(lastname(Gender::Female, "Щусь", Case::Dative), "Щусь");
        assert_eq!(lastname(Gender::Female, "Фидря", Case::Dative), "Фидре");
        assert_eq!(
            lastname(Gender::Female, "Белоконь", Case::Dative),
            "Белоконь"
        );
        assert_eq!(
            lastname(Gender::Female, "Добробаба", Case::Dative),
            "Добробабе"
        );
        assert_eq!(
            lastname(Gender::Female, "Исайченко", Case::Dative),
            "Исайченко"
        );
        assert_eq!(
            lastname(Gender::Female, "Бондаришин", Case::Dative),
            "Бондаришин"
        );
        assert_eq!(lastname(Gender::Female, "Дубинка", Case::Dative), "Дубинке");
        assert_eq!(lastname(Gender::Female, "Сирота", Case::Dative), "Сироте");
        assert_eq!(lastname(Gender::Female, "Воевода", Case::Dative), "Воеводе");
        assert_eq!(lastname(Gender::Female, "Гулыга", Case::Dative), "Гулыге");
        assert_eq!(lastname(Gender::Female, "Дейнека", Case::Dative), "Дейнеке");
        assert_eq!(lastname(Gender::Female, "Цой", Case::Dative), "Цой");
        assert_eq!(lastname(Gender::Female, "Вий", Case::Dative), "Вий");
        assert_eq!(
            lastname(Gender::Female, "Джанджагава", Case::Dative),
            "Джанджагаве"
        );
        assert_eq!(
            lastname(Gender::Female, "Забейворота", Case::Dative),
            "Забейворота"
        );
        assert_eq!(
            lastname(Gender::Female, "Окуджава", Case::Dative),
            "Окуджаве"
        );
        assert_eq!(
            lastname(Gender::Female, "Иванова-Сидорова", Case::Dative),
            "Ивановой-Сидоровой"
        );
    }

    #[test]
    fn should_detect_gender() {
        assert_eq!(detect_gender(None, None, None), Gender::Androgynous);
        assert_eq!(detect_gender(None, Some("Александр"), None), Gender::Male);
        assert_eq!(
            detect_gender(Some("Склифасовский"), None, None),
            Gender::Male
        );
        assert_eq!(
            detect_gender(None, Some("Александра"), None),
            Gender::Female
        );
        assert_eq!(
            detect_gender(Some("Склифасовская"), None, None),
            Gender::Female
        );
        assert_eq!(
            detect_gender(Some("Склифасовская"), Some("Александра"), None),
            Gender::Female
        );
        assert_eq!(detect_gender(None, Some("Саша"), None), Gender::Androgynous);
        assert_eq!(
            detect_gender(Some("Андрейчук"), Some("Саша"), None),
            Gender::Androgynous
        );
        assert_eq!(
            detect_gender(Some("Иванов"), Some("Саша"), None),
            Gender::Male
        );
        assert_eq!(
            detect_gender(Some("Андрейчук"), Some("Саша"), Some("Олегович")),
            Gender::Male
        );
        assert_eq!(
            detect_gender(None, Some("Саша"), Some("Олегович")),
            Gender::Male
        );
        assert_eq!(
            detect_gender(Some("Осипчук"), None, None),
            Gender::Androgynous
        );
        assert_eq!(detect_gender(None, None, Some("Олегович")), Gender::Male);
        assert_eq!(detect_gender(None, None, Some("Олеговна")), Gender::Female);
        assert_eq!(detect_gender(None, None, Some("Сергеевич")), Gender::Male);
        assert_eq!(detect_gender(None, None, Some("Степаныч")), Gender::Male);
        assert_eq!(detect_gender(None, None, Some("Петровна")), Gender::Female);
        assert_eq!(detect_gender(None, None, Some("Оно")), Gender::Androgynous);
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_apis() {
        let subject = Petrovich::new();
        assert_eq!(Petrovich::detect_gender("Валентиновна"), Gender::Female);
        assert_eq!(
            subject.firstname(Gender::Male, "", Case::Genitive).unwrap(),
            ""
        );
        assert_eq!(
            subject
                .firstname(Gender::Male, "Андрей", Case::Genitive)
                .unwrap(),
            "Андрея"
        );
        assert_eq!(
            subject
                .middlename(Gender::Male, "", Case::Genitive)
                .unwrap(),
            ""
        );
        assert_eq!(
            subject
                .middlename(Gender::Male, "Порфирьевич", Case::Genitive)
                .unwrap(),
            "Порфирьевича"
        );
        assert_eq!(
            subject.lastname(Gender::Male, "", Case::Genitive).unwrap(),
            ""
        );
        assert_eq!(
            subject
                .lastname(Gender::Male, "Свистоплясов", Case::Genitive)
                .unwrap(),
            "Свистоплясова"
        );
    }
}
