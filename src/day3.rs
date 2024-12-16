use regex::Regex;

// #[aoc_generator(day3)]
// fn parse(input: &str) -> String {
//     todo!()
// }

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result = re.captures_iter(input).map(|c| {
        let (_, [first, second]) = c.extract();

        let first: u32 = first.parse().unwrap();
        let second: u32 = second.parse().unwrap();

        first * second
    }).sum();

    return result;
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let (_, result) = re.captures_iter(input).fold((true, 0), |acc, c| {
        let (active, sum) = acc;

        match &c[0] {
            "do()" => {
                (true, sum)
            }
            "don't()" => {
                (false, sum)
            }
            _ => {
                if !active {
                    (active, sum)
                } else {
                    let first: &str = &c[1];
                    let second: &str = &c[2];

                    let first: u32 = first.parse().unwrap();
                    let second: u32 = second.parse().unwrap();

                    (true, sum + first * second)
                }
                
            }
        }
    });

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let output = part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(output, 161);
    }

    #[test]
    fn part2_example() {
        let output = part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(output, 48);
    }
}
