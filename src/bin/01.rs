advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .split('\n')
            .filter_map(|line| line.parse::<i64>().ok())
            .map(get_fuel_cost)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .split('\n')
            .filter_map(|line| line.parse::<i64>().ok())
            .map(get_total_fuel_cost)
            .sum(),
    )
}

fn get_total_fuel_cost(mut mass: i64) -> i64 {
    let mut res = 0i64;
    while get_fuel_cost(mass) > 0 {
        res += get_fuel_cost(mass);
        mass = get_fuel_cost(mass);
    }
    res
}

fn get_fuel_cost(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34239));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2 + 966 + 50346));
    }
}
