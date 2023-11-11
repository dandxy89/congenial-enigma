use std::path::PathBuf;

use congenial_enigma::{
    model::{LPDefinition, Sense},
    parse::{parse_file, parse_lp_file},
};

#[test]
fn afiro() {
    let result = read_file_from_resources("afiro.lp").unwrap();
    assert_eq!("afiro.mps", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 3);
    assert_eq!(result.constraints.len(), 27);
    assert_eq!(result.variables.len(), 44);
}

#[test]
fn afiro_ext() {
    let result = read_file_from_resources("afiro_ext.lp").unwrap();
    assert_eq!("afiro.mps", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 4);
    assert_eq!(result.constraints.len(), 27);
    assert_eq!(result.variables.len(), 66);
}

#[test]
fn boeing1() {
    let result = read_file_from_resources("boeing1.lp").unwrap();
    assert_eq!("boeing1.lp", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 1);
    assert_eq!(result.constraints.len(), 348);
    assert_eq!(result.variables.len(), 856);
}

#[test]
fn boeing2() {
    let result = read_file_from_resources("boeing2.lp").unwrap();
    assert_eq!("boeing2.mps", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 1);
    assert_eq!(result.constraints.len(), 140);
    assert_eq!(result.variables.len(), 280);
}

#[test]
fn fit1d() {
    let result = read_file_from_resources("fit1d.lp").unwrap();
    assert_eq!("fit1d.mps", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 1);
    assert_eq!(result.constraints.len(), 24);
    assert_eq!(result.variables.len(), 2053);
}

// #[test]
// fn fit2d() {
//     let result = read_file_from_resources("fit2d.lp").unwrap();
//     assert_eq!(result.problem_sense, Sense::Minimize);
// }

#[test]
fn kb2() {
    let result = read_file_from_resources("kb2.lp").unwrap();
    assert_eq!("kb2.mps", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 1);
    assert_eq!(result.constraints.len(), 43);
    assert_eq!(result.variables.len(), 79);
}

#[test]
fn pulp() {
    let result = read_file_from_resources("pulp.lp").unwrap();
    assert_eq!("", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 1);
    assert_eq!(result.constraints.len(), 49);
    assert_eq!(result.variables.len(), 86);
}

#[test]
fn pulp2() {
    let result = read_file_from_resources("pulp2.lp").unwrap();
    assert_eq!("", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Maximize);
    assert_eq!(result.objectives.len(), 1);
    assert_eq!(result.constraints.len(), 7);
    assert_eq!(result.variables.len(), 148);
}

#[test]
fn sc50a() {
    let result = read_file_from_resources("sc50a.lp").unwrap();
    assert_eq!("sc50a.lp", result.problem_name);
    assert_eq!(result.problem_sense, Sense::Minimize);
    assert_eq!(result.objectives.len(), 1);
    assert_eq!(result.constraints.len(), 49);
    assert_eq!(result.variables.len(), 70);
}

fn read_file_from_resources(file_name: &str) -> anyhow::Result<LPDefinition> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(format!("resources/{file_name}"));
    let contents = parse_file(&file_path)?;
    parse_lp_file(&contents)
}
