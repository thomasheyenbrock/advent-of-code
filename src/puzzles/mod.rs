mod y2023;
mod y2024;
mod y2025;

use std::time::Instant;

use clap::ValueEnum;

type Solver = fn() -> (String, String);

pub struct Puzzle {
    year: Year,
    day: Day,
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nPuzzle {}/{}", "=".repeat(64), self.year, self.day,)
    }
}

impl Puzzle {
    pub fn new(year: Year, day: Day) -> Self {
        Self { year, day }
    }

    pub fn solve(&self) {
        let solver = match (&self.year, &self.day) {
            (Year::_2023, Day::_01) => y2023::d01::get_solver(),
            (Year::_2023, Day::_02) => y2023::d02::get_solver(),
            (Year::_2023, Day::_03) => y2023::d03::get_solver(),
            (Year::_2023, Day::_04) => y2023::d04::get_solver(),
            (Year::_2023, Day::_05) => y2023::d05::get_solver(),
            (Year::_2023, Day::_06) => y2023::d06::get_solver(),
            (Year::_2023, Day::_07) => y2023::d07::get_solver(),
            (Year::_2023, Day::_08) => y2023::d08::get_solver(),
            (Year::_2023, Day::_09) => y2023::d09::get_solver(),
            (Year::_2024, Day::_01) => y2024::d01::get_solver(),
            (Year::_2024, Day::_02) => y2024::d02::get_solver(),
            (Year::_2024, Day::_03) => y2024::d03::get_solver(),
            (Year::_2024, Day::_04) => y2024::d04::get_solver(),
            (Year::_2024, Day::_05) => y2024::d05::get_solver(),
            (Year::_2024, Day::_06) => y2024::d06::get_solver(),
            (Year::_2024, Day::_07) => y2024::d07::get_solver(),
            (Year::_2024, Day::_08) => y2024::d08::get_solver(),
            (Year::_2024, Day::_09) => y2024::d09::get_solver(),
            (Year::_2024, Day::_10) => y2024::d10::get_solver(),
            (Year::_2024, Day::_11) => y2024::d11::get_solver(),
            (Year::_2024, Day::_12) => y2024::d12::get_solver(),
            (Year::_2024, Day::_13) => y2024::d13::get_solver(),
            (Year::_2024, Day::_14) => y2024::d14::get_solver(),
            (Year::_2024, Day::_15) => y2024::d15::get_solver(),
            (Year::_2024, Day::_16) => y2024::d16::get_solver(),
            (Year::_2024, Day::_17) => y2024::d17::get_solver(),
            (Year::_2024, Day::_18) => y2024::d18::get_solver(),
            (Year::_2024, Day::_19) => y2024::d19::get_solver(),
            (Year::_2024, Day::_20) => y2024::d20::get_solver(),
            (Year::_2024, Day::_21) => y2024::d21::get_solver(),
            (Year::_2024, Day::_22) => y2024::d22::get_solver(),
            (Year::_2024, Day::_23) => y2024::d23::get_solver(),
            (Year::_2024, Day::_24) => y2024::d24::get_solver(),
            (Year::_2024, Day::_25) => y2024::d25::get_solver(),
            (Year::_2025, Day::_01) => y2025::d01::get_solver(),
            (Year::_2025, Day::_02) => y2025::d02::get_solver(),
            (Year::_2025, Day::_03) => y2025::d03::get_solver(),
            (Year::_2025, Day::_04) => y2025::d04::get_solver(),
            (Year::_2025, Day::_05) => y2025::d05::get_solver(),
            (Year::_2025, Day::_06) => y2025::d06::get_solver(),
            (Year::_2025, Day::_07) => y2025::d07::get_solver(),
            (Year::_2025, Day::_08) => y2025::d08::get_solver(),
            (Year::_2025, Day::_09) => y2025::d09::get_solver(),
            (Year::_2025, Day::_10) => y2025::d10::get_solver(),
            (Year::_2025, Day::_11) => y2025::d11::get_solver(),
            (Year::_2025, Day::_12) => y2025::d12::get_solver(),
            _ => return,
        };

        println!("{}", "=".repeat(64));
        println!("Puzzle {}/{}", self.year, self.day);

        let start = Instant::now();
        let (part_1, part_2) = solver();
        let time = start.elapsed();

        println!("  Solution Part 1: {part_1}");
        println!("  Solution Part 2: {part_2}");
        println!("  Time: {time:?}");
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Year {
    _2025,
    _2024,
    _2023,
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Year::_2025 => "2025",
                Year::_2024 => "2024",
                Year::_2023 => "2023",
            }
        )
    }
}

impl Year {
    pub fn all_years() -> Vec<Self> {
        ALL_YEARS.to_vec()
    }

    pub fn all_days(&self) -> Vec<Day> {
        ALL_DAYS.to_vec()
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Day {
    _01,
    _02,
    _03,
    _04,
    _05,
    _06,
    _07,
    _08,
    _09,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _20,
    _21,
    _22,
    _23,
    _24,
    _25,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Day::_01 => "01",
                Day::_02 => "02",
                Day::_03 => "03",
                Day::_04 => "04",
                Day::_05 => "05",
                Day::_06 => "06",
                Day::_07 => "07",
                Day::_08 => "08",
                Day::_09 => "09",
                Day::_10 => "10",
                Day::_11 => "11",
                Day::_12 => "12",
                Day::_13 => "13",
                Day::_14 => "14",
                Day::_15 => "15",
                Day::_16 => "16",
                Day::_17 => "17",
                Day::_18 => "18",
                Day::_19 => "19",
                Day::_20 => "20",
                Day::_21 => "21",
                Day::_22 => "22",
                Day::_23 => "23",
                Day::_24 => "24",
                Day::_25 => "25",
            }
        )
    }
}

const ALL_YEARS: [Year; 3] = [Year::_2025, Year::_2024, Year::_2023];

const ALL_DAYS: [Day; 25] = [
    Day::_01,
    Day::_02,
    Day::_03,
    Day::_04,
    Day::_05,
    Day::_06,
    Day::_07,
    Day::_08,
    Day::_09,
    Day::_10,
    Day::_11,
    Day::_12,
    Day::_13,
    Day::_14,
    Day::_15,
    Day::_16,
    Day::_17,
    Day::_18,
    Day::_19,
    Day::_20,
    Day::_21,
    Day::_22,
    Day::_23,
    Day::_24,
    Day::_25,
];
