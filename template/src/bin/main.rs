use {{crate_name}}::part1::process as process_part1;
use {{crate_name}}::part2::process as process_part2;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result1 = process_part1(file)?;
    println!("part1 -> {}", result1);
    let result2 = process_part2(file)?;
    println!("part2 -> {}", result2);
    Ok(())
}