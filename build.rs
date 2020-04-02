use serde::Deserialize;
use std::io::{BufReader, BufWriter, Write};

#[derive(Deserialize, Debug)]
enum Gender {
    #[serde(rename(deserialize = "male"))]
    Male,
    #[serde(rename(deserialize = "female"))]
    Female,
    #[serde(rename(deserialize = "androgynous"))]
    Androgynous,
}

#[derive(Deserialize, Debug)]
enum RuleTag {
    #[serde(rename(deserialize = "first_word"))]
    FirstWord,
}

#[derive(Deserialize)]
struct Rule {
    gender: Gender,
    test: Vec<String>,
    mods: [String; 5],
    #[serde(default = "Vec::new")]
    tags: Vec<RuleTag>,
}

#[derive(Deserialize)]
struct RuleList {
    exceptions: Vec<Rule>,
    suffixes: Vec<Rule>,
}

#[derive(Deserialize)]
struct Rules {
    lastname: RuleList,
    firstname: RuleList,
    middlename: RuleList,
}

fn generate_rule(rule: &Rule, output: &mut impl Write) -> std::io::Result<()> {
    writeln!(output, "            Rule {{")?;
    writeln!(output, "                gender: Gender::{:?},", rule.gender)?;
    writeln!(output, "                test: &[")?;
    for test in &rule.test {
        writeln!(output, "                    {:?},", test)?;
    }
    writeln!(output, "                ],")?;
    writeln!(output, "                mods: [")?;
    for modifier in rule.mods.iter() {
        if modifier == "." {
            writeln!(output, "                    None,")?;
        } else {
            let dashes: usize = modifier
                .chars()
                .fold(0, |acc, c| if c == '-' { acc + 1 } else { acc });
            let ending = modifier.chars().skip(dashes).collect::<String>();
            writeln!(
                output,
                "                    Some(({}, {:?})),",
                dashes, ending
            )?;
        }
    }
    writeln!(output, "                ],")?;
    writeln!(output, "                tags: &{:?}", &rule.tags)?;
    writeln!(output, "            }},")
}

fn generate_rule_list(list: &RuleList, output: &mut impl Write) -> std::io::Result<()> {
    writeln!(output, "RuleList {{")?;
    writeln!(output, "        exceptions: &[")?;
    for exception in &list.exceptions {
        generate_rule(exception, output)?;
    }
    writeln!(output, "        ],")?;
    writeln!(output, "        suffixes: &[")?;
    for suffix in &list.suffixes {
        generate_rule(suffix, output)?;
    }
    writeln!(output, "        ],")?;
    writeln!(output, "    }},")
}

fn generate_rules(rules: &Rules, output: &mut impl Write) -> std::io::Result<()> {
    writeln!(output, "Rules {{")?;
    write!(output, "    lastname: ")?;
    generate_rule_list(&rules.lastname, output)?;
    write!(output, "    firstname: ")?;
    generate_rule_list(&rules.firstname, output)?;
    write!(output, "    middlename: ")?;
    generate_rule_list(&rules.middlename, output)?;
    writeln!(output, "}}")
}

#[derive(Deserialize)]
struct GenderMapping {
    #[serde(default = "Vec::new")]
    androgynous: Vec<String>,
    #[serde(default = "Vec::new")]
    male: Vec<String>,
    #[serde(default = "Vec::new")]
    female: Vec<String>,
}

#[derive(Deserialize)]
struct GenderHeuristic {
    exceptions: Option<GenderMapping>,
    suffixes: GenderMapping,
}

#[derive(Deserialize)]
struct GenderHeuristics {
    lastname: GenderHeuristic,
    firstname: GenderHeuristic,
    middlename: GenderHeuristic,
}

#[derive(Deserialize)]
struct GenderHeuristicsList {
    gender: GenderHeuristics,
}

fn generate_gender_rules(rules: &[String], output: &mut impl Write) -> std::io::Result<()> {
    for rule in rules {
        writeln!(output, "                {:?},", rule)?;
    }
    Ok(())
}

fn generate_gender_mapping(
    mapping: &GenderMapping,
    output: &mut impl Write,
) -> std::io::Result<()> {
    writeln!(output, "            androgynous: &[")?;
    generate_gender_rules(&mapping.androgynous, output)?;
    writeln!(output, "            ],")?;
    writeln!(output, "            male: &[")?;
    generate_gender_rules(&mapping.male, output)?;
    writeln!(output, "            ],")?;
    writeln!(output, "            female: &[")?;
    generate_gender_rules(&mapping.female, output)?;
    writeln!(output, "            ],")
}

fn generate_gender_heuristic(
    heuristic: &GenderHeuristic,
    output: &mut impl Write,
) -> std::io::Result<()> {
    writeln!(output, "GenderHeuristic {{")?;
    if let Some(mapping) = &heuristic.exceptions {
        writeln!(output, "        exceptions: Some(GenderMapping {{")?;
        generate_gender_mapping(mapping, output)?;
        writeln!(output, "        }}),")?;
    } else {
        writeln!(output, "        exceptions: None,")?;
    }
    writeln!(output, "        suffixes: GenderMapping {{")?;
    generate_gender_mapping(&heuristic.suffixes, output)?;
    writeln!(output, "        }},")?;
    writeln!(output, "    }},")
}

fn generate_gender(gender: &GenderHeuristics, output: &mut impl Write) -> std::io::Result<()> {
    writeln!(output, "GenderHeuristics {{")?;
    write!(output, "    lastname: ")?;
    generate_gender_heuristic(&gender.lastname, output)?;
    write!(output, "    firstname: ")?;
    generate_gender_heuristic(&gender.firstname, output)?;
    write!(output, "    middlename: ")?;
    generate_gender_heuristic(&gender.middlename, output)?;
    writeln!(output, "}}")
}

struct YamlError(serde_yaml::Error);

impl From<YamlError> for std::io::Error {
    fn from(YamlError(error): YamlError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, error)
    }
}

fn main() -> std::io::Result<()> {
    use std::path::Path;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/rules.yml");
    println!("cargo:rerun-if-changed=src/gender.yml");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    let rules_json = std::fs::File::open("src/rules.yml")?;
    let rules = serde_yaml::from_reader(BufReader::new(rules_json)).map_err(YamlError)?;
    let rules_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(Path::new(&out_dir).join("rules.inc"))?;
    generate_rules(&rules, &mut BufWriter::new(rules_file))?;

    let gender_json = std::fs::File::open("src/gender.yml")?;
    let gender: GenderHeuristicsList =
        serde_yaml::from_reader(BufReader::new(gender_json)).map_err(YamlError)?;
    let gender_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(Path::new(&out_dir).join("gender.inc"))?;
    generate_gender(&gender.gender, &mut BufWriter::new(gender_file))
}
