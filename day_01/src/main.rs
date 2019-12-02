use common::input;

fn main() {
    let mut fuel: Vec<u32> = input::read_lines()
        .iter()
        .map(|x| u32::from_str_radix(&x, 10).expect("error converting input"))
        .map(|x| calculate_fuel(x) as u32)
        .collect();

    println!(
        "[PART ONE] sum of the fuel requirements: {}",
        fuel.iter().sum::<u32>()
    );

    for f in fuel.iter_mut() {
        *f += calculate_fuel_recursive(*f);
    }

    println!(
        "[PART TWO] sum of the fuel requirements: {}",
        fuel.iter().sum::<u32>()
    );
}

fn calculate_fuel(mass: u32) -> i32 {
    (mass as f32 / 3_f32).floor() as i32 - 2
}

fn calculate_fuel_recursive(mass: u32) -> u32 {
    let fuel = calculate_fuel(mass);

    if fuel.is_negative() {
        return 0;
    }

    fuel as u32 + calculate_fuel_recursive(fuel as u32)
}

#[cfg(test)]
mod tests {
    use super::{calculate_fuel, calculate_fuel_recursive};

    #[test]
    fn part_one() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn part_two() {
        assert_eq!(calculate_fuel_recursive(14), 2);
        assert_eq!(calculate_fuel_recursive(1969), 966);
        assert_eq!(calculate_fuel_recursive(100756), 50346);
    }
}
