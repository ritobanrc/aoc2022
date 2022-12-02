mod day01;
mod day02;
//mod day03;
//mod day04;
//mod day05;
//mod day06;
//mod day07;
//mod day08;
//mod day09;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day17;

pub type DayFn = fn(&str) -> Box<dyn std::fmt::Debug>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

macro_rules! aoc {
    (with_enum:$day: expr) => {
        paste::item! {
            Some((|input| Box::new([<day $day>]::solutions(&input, Part::Part1)), |input| Box::new([<day $day>]::solutions(&input, Part::Part2))))
        }
    };
    (with_enum:$day: expr, $ans1: expr) => {
        Some((|input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { Box::new([<day $day>]::solutions(&input, Part::Part2)) }
        }
        ))
    };
    (with_enum:$day: expr, $ans1: expr, $ans2: expr) => {
        Some((|input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part2); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        }))
    };


    ($day: expr) => {
        paste::item! {
            Some((|input| Box::new([<day $day>]::part1(&input)), |input| Box::new([<day $day>]::part2(&input))))
        }
    };
    ($day: expr, $ans1: expr) => {
        paste::item! {
            Some((|input| {
                let ans =[<day $day>]::part1(&input);
                assert_eq!(ans, $ans1);
                Box::new(ans)
            }, |input| Box::new([<day $day>]::part2(&input))))
        }
    };
    ($day: expr, $ans1: expr, $ans2: expr) => {
        Some((|input| {
            paste::item! { let ans = [<day $day>]::part1(&input); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans =[<day $day>]::part2(&input); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        }))
    };
}

pub fn get_day(day: u32) -> Option<(DayFn, DayFn)> {
    return match day {
        1 => aoc!(01, 71780, 212489),
        2 => aoc!(02, 14069, 12411),
        _ => {
            eprintln!("Unknown day: {}", day);
            return None;
        }
    };
}
