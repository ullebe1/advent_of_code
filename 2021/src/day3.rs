use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let lines = input.lines().into_iter();
    let nr_of_lines = lines.clone().count();
    let mut index_values: Vec<usize> = Vec::new();
    for line in lines {
        if index_values.is_empty() {
            index_values = vec![0; line.len()]
        }
        for (index, char) in line.chars().enumerate() {
            index_values[index] = index_values[index] + char.to_digit(2).unwrap() as usize;
        }
    }
    let mut gamma = String::new();
    for value in index_values {
        gamma.push_str(&format!("{}", (value / (nr_of_lines / 2)) as usize));
    }
    println!("{}", gamma);
    let epsilon = usize::from_str_radix(
        &gamma.replace("1", "2").replace("0", "1").replace("2", "0"),
        2,
    )
    .unwrap();
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();

    return gamma * epsilon;
}

enum lifeSupport {
    CO2,
    Oxygen,
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let lines = input.lines().into_iter().collect::<Vec<&str>>();
    let oxygen_rating = filter_number_list(lines.clone(), 0, lifeSupport::Oxygen);
    let co2_rating = filter_number_list(lines, 0, lifeSupport::CO2);
    return oxygen_rating * co2_rating;
}

fn filter_number_list(numbers: Vec<&str>, index: usize, version: lifeSupport) -> usize {
    let mut ones: Vec<&str> = Vec::new();
    let mut zeros: Vec<&str> = Vec::new();

    for number in numbers {
        let digits = number.chars().collect::<Vec<char>>();
        match digits[index].to_string().as_str() {
            "0" => zeros.push(number),
            "1" => ones.push(number),
            _ => unreachable!(),
        }
    }
    let selected_vec = match version {
        lifeSupport::CO2 => {
            if zeros.len() <= ones.len() {
                zeros
            } else {
                ones
            }
        }
        lifeSupport::Oxygen => {
            if ones.len() >= zeros.len() {
                ones
            } else {
                zeros
            }
        }
    };
    if selected_vec.len() == 1 {
        return usize::from_str_radix(selected_vec[0], 2).unwrap();
    } else {
        return filter_number_list(selected_vec, index + 1, version);
    }
}
