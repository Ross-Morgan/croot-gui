use croot::prelude::*;
use num_complex::Complex64;
use poloto::build::{self, PlotIterator};

static SUBSCRIPT: [char; 10] = ['₀' '₁' '₂' '₃' '₄' '₅' '₆' '₇' '₈' '₉'];

fn poloto_graph() {
    let roots = root(32.0, 5);
    let argand_lines = line_functions(roots.as_slice());
    
}

fn line_functions(roots: &[Complex64]) -> impl Iterator<Item = (String, )> {
    roots
        .iter()
        .enumerate()
        .map(|(idx, root)| {
            let label = format!("{} = {}", root_notation("ℂ", idx), root.to_string());
            
            let range = poloto::util::range_iter([0.0, root.re], 200);
    
            (label, )
        })
}
    
fn root_notation(symbol: &str, n: usize) -> String {
    0..(n.log10().ceil() as usize)
        .map(|i| nth_digit(n, i))
}
    
const fn nth_digit(k: usize, n: usize) -> usize {
    (k / 10.pow(n)) % 10
}
