use {{crate_name}}::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
