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
//!     assert_eq!(firstname(Gender::Male, "Саша", Case::Dative).unwrap(),
//!                "Саше");
//!     assert_eq!(firstname(Gender::Female, "Изабель", Case::Genitive).unwrap(),
//!                "Изабель");
//!
//!     assert_eq!(lastname(Gender::Male, "Станкевич", Case::Prepositional).unwrap(),
//!                "Станкевиче");
//!     assert_eq!(lastname(Gender::Female, "Станкевич", Case::Prepositional).unwrap(),
//!                "Станкевич");
//!
//!     assert_eq!(middlename(Gender::Male, "Сергеич", Case::Instrumental).unwrap(),
//!                "Сергеичем");
//!     assert_eq!(middlename(Gender::Female, "Прокопьевна", Case::Accusative).unwrap(),
//!                "Прокопьевну");
//! }
//! ```

/// Возможные рода
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Gender {
    /// Мужской род
    Male,
    /// Женский род
    Female,
    /// Средний род
    Androgynous,
}

struct Rule {
    gender: Gender,
    test: &'static [&'static str],
    mods: [&'static str; 5],
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
fn find_exception<'a>(exceptions: &'a [Rule], name: &str, gender: Gender) -> Option<&'a Rule> {
    // Search exceptions with matching name and gender
    exceptions.iter().find(|&exception| {
        // Check if name matches
        let does_match_test = exception
            .test
            .iter()
            .any(|&test| test == name.to_lowercase());

        // Check if gender matches
        let does_match_gender = exception.gender == gender || exception.gender == Androgynous;

        // Return true if both match
        does_match_test && does_match_gender
    })
}

// Find suffix by name and gender
fn find_suffix<'a>(suffixes: &'a [Rule], name: &str, gender: Gender) -> Option<&'a Rule> {
    suffixes
        .iter()
        .filter(|&suffix| {
            // Check if suffix matches
            let does_match_test = suffix
                .test
                .iter()
                .any(|&test| name.to_lowercase().ends_with(test));

            // Check if gender matches
            let does_match_gender = suffix.gender == gender || suffix.gender == Androgynous;

            // Return true if both match
            does_match_test && does_match_gender
        })
        .max_by_key(|&rule| {
            // Find longest matching
            rule.test
                .iter()
                .filter(|&&test| name.to_lowercase().ends_with(test))
                .max_by_key(|&&test| test.len())
                .unwrap()
                .len()
        })
}

fn inflect(name: &str, rule: &Rule, case: Case) -> String {
    // Get inflection by case
    let inflection = rule.mods[case as usize];

    // Count amount of dashes: "-" thus amount of characters left remaining
    let remaining: usize = name.chars().count() - inflection.rfind("-").map_or(0, |pos| pos + 1);

    let matches: &[_] = &['-', '.'];
    let postfix = inflection.trim_start_matches(matches);

    // Apply inflection
    return name.chars().take(remaining).collect::<String>() + postfix;
}

fn inflect_name(
    gender: Gender,
    name: &str,
    case: Case,
    rule_list: &RuleList,
) -> Result<String, &'static str> {
    // First let's check for exceptions
    find_exception(rule_list.exceptions, name, gender)
        // Then check for suffixes
        .or(find_suffix(rule_list.suffixes, name, gender))
        // If no match found, return error
        .ok_or("No matching rule found")
        // Then inflect name using matched rule
        .and_then(|rule| Ok(inflect(name, rule, case)))
}

/// Inflects first name
pub fn firstname(gender: Gender, name: &str, case: Case) -> Result<String, &'static str> {
    inflect_name(gender, name, case, &RULES.firstname)
}

/// Inflects last name
pub fn lastname(gender: Gender, name: &str, case: Case) -> Result<String, &'static str> {
    inflect_name(gender, name, case, &RULES.lastname)
}

/// Inflects middle name
pub fn middlename(gender: Gender, name: &str, case: Case) -> Result<String, &'static str> {
    inflect_name(gender, name, case, &RULES.middlename)
}

/// Detects gender of a middlename, fallbacks to `Gender::Androgynous`
pub fn detect_gender(middlename: &str) -> Gender {
    if middlename.ends_with("ич") || middlename.ends_with("ыч") {
        return Gender::Male;
    }

    if middlename.ends_with("на") {
        return Gender::Female;
    }

    return Gender::Androgynous;
}

pub mod deprecated;
use crate::Gender::Androgynous;
pub use deprecated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_error() {
        assert!(firstname(Gender::Male, "Blabla", Case::Genitive).is_err());
        assert!(middlename(Gender::Male, "Blabla", Case::Genitive).is_err());
        assert!(lastname(Gender::Male, "Blabla", Case::Genitive).is_err());
    }

    #[test]
    fn should_inflect_first_names() {
        assert_eq!(
            firstname(Gender::Male, "Лёша", Case::Genitive).unwrap(),
            "Лёши"
        );
        assert_eq!(
            firstname(Gender::Male, "Лёша", Case::Dative).unwrap(),
            "Лёше"
        );
        assert_eq!(
            firstname(Gender::Male, "Лёша", Case::Accusative).unwrap(),
            "Лёшу"
        );
        assert_eq!(
            firstname(Gender::Male, "Лёша", Case::Instrumental).unwrap(),
            "Лёшей"
        );
        assert_eq!(
            firstname(Gender::Male, "Лёша", Case::Prepositional).unwrap(),
            "Лёше"
        );

        assert_eq!(
            firstname(Gender::Male, "Яша", Case::Genitive).unwrap(),
            "Яши"
        );
        assert_eq!(firstname(Gender::Male, "Яша", Case::Dative).unwrap(), "Яше");
        assert_eq!(
            firstname(Gender::Male, "Яша", Case::Accusative).unwrap(),
            "Яшу"
        );
        assert_eq!(
            firstname(Gender::Male, "Яша", Case::Instrumental).unwrap(),
            "Яшей"
        );
        assert_eq!(
            firstname(Gender::Male, "Яша", Case::Prepositional).unwrap(),
            "Яше"
        );
    }

    #[test]
    fn should_inflect_complex_male_lastnames() {
        assert_eq!(
            lastname(Gender::Male, "Бильжо", Case::Dative).unwrap(),
            "Бильжо"
        );
        assert_eq!(
            lastname(Gender::Male, "Ничипорук", Case::Dative).unwrap(),
            "Ничипоруку"
        );
        assert_eq!(
            lastname(Gender::Male, "Щусь", Case::Dative).unwrap(),
            "Щусю"
        );
        assert_eq!(
            lastname(Gender::Male, "Фидря", Case::Dative).unwrap(),
            "Фидре"
        );
        assert_eq!(
            lastname(Gender::Male, "Белоконь", Case::Dative).unwrap(),
            "Белоконю"
        );
        assert_eq!(
            lastname(Gender::Male, "Добробаба", Case::Dative).unwrap(),
            "Добробабе"
        );
        assert_eq!(
            lastname(Gender::Male, "Исайченко", Case::Dative).unwrap(),
            "Исайченко"
        );
        assert_eq!(
            lastname(Gender::Male, "Бондаришин", Case::Dative).unwrap(),
            "Бондаришину"
        );
        assert_eq!(
            lastname(Gender::Male, "Дубинка", Case::Dative).unwrap(),
            "Дубинке"
        );
        assert_eq!(
            lastname(Gender::Male, "Сирота", Case::Dative).unwrap(),
            "Сироте"
        );
        assert_eq!(
            lastname(Gender::Male, "Воевода", Case::Dative).unwrap(),
            "Воеводе"
        );
        assert_eq!(
            lastname(Gender::Male, "Волож", Case::Dative).unwrap(),
            "Воложу"
        );
        assert_eq!(
            lastname(Gender::Male, "Кравец", Case::Dative).unwrap(),
            "Кравцу"
        );
        assert_eq!(
            lastname(Gender::Male, "Самотечний", Case::Dative).unwrap(),
            "Самотечнему",
        );
        assert_eq!(lastname(Gender::Male, "Цой", Case::Dative).unwrap(), "Цою");
        assert_eq!(
            lastname(Gender::Male, "Шопен", Case::Dative).unwrap(),
            "Шопену"
        );
        assert_eq!(
            lastname(Gender::Male, "Сосковец", Case::Dative).unwrap(),
            "Сосковцу"
        );
    }

    #[test]
    fn should_inflect_complex_female_lastnames() {
        assert_eq!(
            lastname(Gender::Female, "Бильжо", Case::Dative).unwrap(),
            "Бильжо"
        );
        assert_eq!(
            lastname(Gender::Female, "Ничипорук", Case::Dative).unwrap(),
            "Ничипорук"
        );
        assert_eq!(
            lastname(Gender::Female, "Щусь", Case::Dative).unwrap(),
            "Щусь"
        );
        assert_eq!(
            lastname(Gender::Female, "Фидря", Case::Dative).unwrap(),
            "Фидре"
        );
        assert_eq!(
            lastname(Gender::Female, "Белоконь", Case::Dative).unwrap(),
            "Белоконь"
        );
        assert_eq!(
            lastname(Gender::Female, "Добробаба", Case::Dative).unwrap(),
            "Добробабе"
        );
        assert_eq!(
            lastname(Gender::Female, "Исайченко", Case::Dative).unwrap(),
            "Исайченко"
        );
        assert_eq!(
            lastname(Gender::Female, "Бондаришин", Case::Dative).unwrap(),
            "Бондаришин"
        );
        assert_eq!(
            lastname(Gender::Female, "Дубинка", Case::Dative).unwrap(),
            "Дубинке"
        );
        assert_eq!(
            lastname(Gender::Female, "Сирота", Case::Dative).unwrap(),
            "Сироте"
        );
        assert_eq!(
            lastname(Gender::Female, "Воевода", Case::Dative).unwrap(),
            "Воеводе"
        );
        assert_eq!(
            lastname(Gender::Female, "Гулыга", Case::Dative).unwrap(),
            "Гулыге"
        );
        assert_eq!(
            lastname(Gender::Female, "Дейнека", Case::Dative).unwrap(),
            "Дейнеке"
        );
        assert_eq!(
            lastname(Gender::Female, "Джанджагава", Case::Dative).unwrap(),
            "Джанджагаве"
        );
        assert_eq!(
            lastname(Gender::Female, "Забейворота", Case::Dative).unwrap(),
            "Забейворота"
        );
        assert_eq!(
            lastname(Gender::Female, "Окуджава", Case::Dative).unwrap(),
            "Окуджаве"
        );
    }

    #[test]
    fn should_detect_gender() {
        assert_eq!(detect_gender("Сергеевич"), Gender::Male);
        assert_eq!(detect_gender("Степаныч"), Gender::Male);
        assert_eq!(detect_gender("Петровна"), Gender::Female);
        assert_eq!(detect_gender("Оно"), Gender::Androgynous);
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_apis() {
        let subject = Petrovich::new();
        assert_eq!(Petrovich::detect_gender("Валентиновна"), Gender::Female);
        assert_eq!(
            subject
                .firstname(Gender::Male, "", Case::Genitive)
                .unwrap_err(),
            "No matching rule found"
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
                .unwrap_err(),
            "No matching rule found"
        );
        assert_eq!(
            subject
                .middlename(Gender::Male, "Порфирьевич", Case::Genitive)
                .unwrap(),
            "Порфирьевича"
        );
        assert_eq!(
            subject
                .lastname(Gender::Male, "", Case::Genitive)
                .unwrap_err(),
            "No matching rule found"
        );
        assert_eq!(
            subject
                .lastname(Gender::Male, "Свистоплясов", Case::Genitive)
                .unwrap(),
            "Свистоплясова"
        );
    }
}
