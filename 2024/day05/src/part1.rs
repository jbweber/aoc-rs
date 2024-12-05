use anyhow::bail;
use std::collections::HashMap;

pub fn process(input: &str) -> anyhow::Result<String> {
    let parts = input.replace("\r\n", "\n");
    let parts = parts.split("\n\n").collect::<Vec<_>>();

    if parts.len() != 2 {
        bail!("Invalid number of segments: {}", parts.len());
    }

    let mut mapping = HashMap::new();

    parts[0].lines().for_each(|line| {
        if let Some((key, value)) = line.split_once('|') {
            let key: u32 = key.parse().expect("Invalid key");
            let value: u32 = value.parse().expect("Invalid value");
            mapping.entry(key).or_insert_with(Vec::new).push(value);
        }
    });

    let prints = parts[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|i| i.parse::<u32>().expect("Invalid number"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let good_prints = prints
        .iter()
        .filter(|print| {
            // for each item in print
            for (idx, &value) in print.iter().enumerate() {
                // check there is no rule which says the item must be before a previous item
                for check in print.iter().skip(idx + 1) {
                    if mapping.contains_key(check) && mapping[check].contains(&value) {
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>();

    let result = good_prints.iter().map(|p| p[p.len() / 2]).sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
