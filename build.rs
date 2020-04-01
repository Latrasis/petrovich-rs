use serde::Deserialize;
use std::io::{BufWriter, Write};

#[derive(Deserialize, Debug)]
enum Gender {
    #[serde(rename(deserialize = "male"))]
    Male,
    #[serde(rename(deserialize = "female"))]
    Female,
    #[serde(rename(deserialize = "androgynous"))]
    Androgynous,
}

#[derive(Deserialize)]
struct Rule {
    gender: Gender,
    test: Vec<String>,
    mods: [String; 5],
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
    writeln!(output, "                mods: {:?},", rule.mods)?;
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
    writeln!(output, "    }},")?;
    Ok(())
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

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let rules_path = std::path::Path::new(&out_dir).join("rules.inc");

    let rules_json = std::fs::File::open("src/rules.json")?;
    let rules = serde_json::from_reader(std::io::BufReader::new(rules_json))?;

    let rules_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(rules_path)?;

    generate_rules(&rules, &mut BufWriter::new(rules_file))
}
