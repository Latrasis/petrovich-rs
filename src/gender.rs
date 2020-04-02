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

struct GenderMapping {
    androgynous: &'static [&'static str],
    male: &'static [&'static str],
    female: &'static [&'static str],
}

struct GenderHeuristic {
    exceptions: Option<GenderMapping>,
    suffixes: GenderMapping,
}

impl GenderHeuristic {
    fn detect_gender(&self, name: &str) -> Option<Gender> {
        let find_exception = |exceptions: &[&str]| exceptions.contains(&name);
        let find_suffix = |suffixes: &[&str]| suffixes.iter().any(|&suffix| name.ends_with(suffix));
        self.exceptions
            .as_ref()
            .and_then(|mapping| {
                if find_exception(mapping.androgynous) {
                    None
                } else if find_exception(mapping.female) {
                    Some(Gender::Female)
                } else if find_exception(mapping.male) {
                    Some(Gender::Male)
                } else {
                    None
                }
            })
            .or_else(|| {
                if find_suffix(self.suffixes.androgynous) {
                    None
                } else if find_suffix(self.suffixes.female) {
                    Some(Gender::Female)
                } else if find_suffix(self.suffixes.male) {
                    Some(Gender::Male)
                } else {
                    None
                }
            })
    }
}

struct GenderHeuristics {
    lastname: GenderHeuristic,
    firstname: GenderHeuristic,
    middlename: GenderHeuristic,
}

const GENDER: GenderHeuristics = include!(concat!(env!("OUT_DIR"), "/gender.inc"));

/// Detects gender of a middlename, fallbacks to `Gender::Androgynous`
pub fn detect_gender(
    lastname: Option<&str>,
    firstname: Option<&str>,
    middlename: Option<&str>,
) -> Gender {
    middlename
        .and_then(|middlename| GENDER.middlename.detect_gender(&middlename.to_lowercase()))
        .or_else(|| {
            firstname
                .and_then(|firstname| GENDER.firstname.detect_gender(&firstname.to_lowercase()))
        })
        .or_else(|| {
            lastname.and_then(|lastname| GENDER.lastname.detect_gender(&lastname.to_lowercase()))
        })
        .unwrap_or(Gender::Androgynous)
}
