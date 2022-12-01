pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut nums: Vec<_> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<usize>().unwrap()).sum())
        .collect();
    nums.sort_by_key(|&x| std::cmp::Reverse(x));
    nums[0..3].iter().sum()
}
