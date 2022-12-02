use itertools::Itertools;

fn calc_score(opp: u8, you: u8) -> usize {
    let win_score = if opp == you {
        // draw
        3
    } else if you == (opp + 1) % 3 {
        6
    } else {
        0
    };
    (you + 1) as usize + win_score
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let (opp, you) = l.split(' ').collect_tuple()?;
            let opp = opp.bytes().next()? - b'A';
            let you = you.bytes().next()? - b'X';
            Some(calc_score(opp, you))
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let (opp, you) = l.split(' ').collect_tuple()?;
            let opp = opp.bytes().next()? - b'A';
            let target = you.bytes().next()? - b'X';
            let you = match target {
                0 => (3 + opp - 1) % 3,
                1 => opp, // draw
                2 => (opp + 1) % 3,
                _ => panic!("invalid target"),
            };
            Some(calc_score(opp, you))
        })
        .sum()
}
