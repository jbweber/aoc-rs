use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("190: 10 19", (190, vec![10,19]))]
    #[case("3267: 81 40 27", (3267, vec![81,40,27]))]
    #[case("83: 17 5", (83, vec![17,5]))]
    #[case("156: 15 6", (156, vec![15,6]))]
    #[case("7290: 6 8 6 15", (7290, vec![6,8,6,15]))]
    #[case("161011: 16 10 13", (161011, vec![16,10,13]))]
    #[case("192: 17 8 14", (192, vec![17,8,14]))]
    #[case("21037: 9 7 18 13", (21037, vec![9,7,18,13]))]
    #[case("292: 11 6 16 20", (292, vec![11,6,16,20]))]
    fn test_parse_input(#[case] input: &str, #[case] expected: (u64, Vec<u64>)) {
        assert_eq!(parse_input(input).unwrap().1[0], expected);
    }
}
