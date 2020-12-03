use tools::{self, Error};

fn count_trees(rows: &[String], dx: usize, dy: usize) -> usize {
    let mut x = 0;
    rows.iter()
        .step_by(dy)
        .filter(|row| {
            let idx = x;
            x = (x + dx) % row.len();
            row.chars().nth(idx) == Some('#')
        })
        .count()
}

fn main() -> Result<(), Error> {
    let rows = tools::read_input("input.txt")?;
    println!("{}", count_trees(&rows, 3, 1));
    println!(
        "{}",
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(dx, dy)| count_trees(&rows, *dx, *dy))
            .product::<usize>()
    );
    Ok(())
}
