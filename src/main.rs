use std::fs;

use anyhow::{Context, Result};
use crustabri::aa::{AAFramework, Semantics};
use crustabri::io::{Iccma23Reader, InstanceReader};
use crustabri::solvers::{PreferredSemanticsSolver, SkepticalAcceptanceComputer};

fn decide_preferred(af: &AAFramework<usize>, id: usize) -> bool {
    PreferredSemanticsSolver::new(af).are_skeptically_accepted(&[&id])
}

fn run<P: AsRef<std::path::Path>>(path: P, id: usize, semantics: Semantics) -> Result<bool> {
    let mut file = fs::File::open(path).context("opening af file")?;
    let reader = Iccma23Reader::default();
    let af = reader.read(&mut file)?;

    Ok(match semantics {
        Semantics::PR => decide_preferred(&af, id),
        _ => todo!(),
    })
}

fn main() {
    let path = std::env::args().nth(1).expect("missing argument");
    let id = std::env::args().nth(2).expect("missing argument");
    let semantics = std::env::args().nth(3).expect("missing argument");

    let id = id.parse::<usize>().expect("invalid id");
    let semantics = Semantics::try_from(semantics.as_str()).expect("invalid semantics");

    let result = run(path, id, semantics).expect("error running solver");
    println!("FINAL RESULT: {}", result);
}
