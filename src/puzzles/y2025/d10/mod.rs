use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn gcd(a: isize, b: isize) -> isize {
    let a = a.abs();
    let b = b.abs();
    if a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn lcm(a: isize, b: isize) -> isize {
    let a = a.abs();
    let b = b.abs();
    a * b / gcd(a, b)
}

fn toggle(states: &Vec<&Vec<usize>>) -> Vec<usize> {
    let mut result = vec![0; states[0].len()];
    for state in states {
        for (i, s) in state.iter().enumerate() {
            match (&result[i], s) {
                (0, 1) => result[i] = 1,
                (1, 1) => result[i] = 0,
                _ => {}
            }
        }
    }
    result
}

struct Machine {
    lights: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" ").peekable();

        let mut lights_str = iter.next().unwrap().to_string();
        lights_str.remove(0);
        lights_str.pop();
        let lights = lights_str
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                c => panic!("Unknown light {c}"),
            })
            .collect::<Vec<_>>();

        let mut buttons = vec![];
        let mut joltage = vec![];
        loop {
            let mut item = iter.next().unwrap().to_string();
            item.remove(0);
            item.pop();

            if iter.peek().is_none() {
                joltage.extend(item.split(",").map(|s| s.parse::<usize>().unwrap()));
                break;
            }

            let mut button = vec![0; lights.len()];

            for index in item.split(",") {
                let index = index.parse::<usize>().unwrap();
                button[index] = 1;
            }

            buttons.push(button);
        }

        Self {
            lights,
            buttons,
            joltage,
        }
    }
}

impl Machine {
    fn toggle_lights(&self) -> usize {
        let button_count = self.buttons.len();
        let mut min_count = button_count + 1;
        let mut found_match = false;

        for permutation in 1..2u32.pow(button_count as u32) {
            let mut selection = Vec::with_capacity(button_count);

            for i in 0..button_count {
                if permutation & (1 << i) != 0 {
                    selection.push(&self.buttons[i]);
                }
            }

            if toggle(&selection) == self.lights {
                found_match = true;
                min_count = min_count.min(selection.len());
            }
        }

        if !found_match {
            panic!("No match for machine")
        }
        min_count
    }

    fn set_joltage(&self) -> usize {
        let max_presses = *self.joltage.iter().max().unwrap() as isize;

        let mut equation_system = EquationSystem::from(self);
        equation_system.solve();

        for (i, (coefficients, constant)) in equation_system.equations.iter().enumerate() {
            // Assert all diagonal coefficients are equal to the multiplier
            if coefficients[i] != equation_system.multiplier {
                panic!("Diagonal coefficients must be positive");
            }

            // Assert all non-diagonal coefficients are zero
            for j in 0..equation_system.coefficient_count {
                if j != i && coefficients[j] != 0 {
                    panic!("Non-diagonal coefficients must be zero");
                }
            }

            // Assert all literal constants are positive
            if let Some(a) = constant.as_literal() {
                if a < 0 {
                    panic!("Constants must be positive");
                }
            }
        }

        let constant_sum = equation_system
            .equations
            .iter()
            .fold(Constant(vec![]), |sum, (_, constant)| {
                sum + constant.clone()
            });
        if let Some(sum) = constant_sum.as_literal() {
            return (sum / equation_system.multiplier) as usize;
        }

        let mut min_max_values = HashMap::new();
        for variable in equation_system.variables.iter() {
            let mut min_value = 0;
            #[allow(unused_assignments)]
            let mut max_value = 0;

            let mut looking_for_min = true;
            let mut i = 0;
            loop {
                let no_negatives = equation_system.equations.iter().all(|(_, constant)| {
                    match constant.resolve(variable, i).as_literal() {
                        Some(a) if a < 0 => false,
                        _ => true,
                    }
                });
                if no_negatives && looking_for_min {
                    min_value = i;
                    looking_for_min = false;
                }
                if !no_negatives && !looking_for_min {
                    max_value = i - 1;
                    break;
                }

                i += 1;

                if i == max_presses {
                    max_value = i;
                    break;
                }
            }

            min_max_values.insert(variable, (min_value, max_value));
        }

        let mut min_presses: Option<isize> = None;

        // Run through all possible values for variables to find the combination that results in the minimum number of presses
        let mut values = vec![min_max_values.get(&equation_system.variables[0]).unwrap().0];
        let mut prev_constants: Option<Vec<isize>> = None;
        while values.len() != 0 {
            if values.len() < equation_system.variables.len() {
                values.push(
                    min_max_values
                        .get(&equation_system.variables[values.len()])
                        .unwrap()
                        .0,
                );
                continue;
            }

            let variable_values = values
                .iter()
                .enumerate()
                .map(|(i, value)| (&equation_system.variables[i], *value))
                .collect::<Vec<_>>();
            let constants = equation_system
                .equations
                .iter()
                .map(|(_, constant)| constant.resolve_all(&variable_values).as_literal().unwrap())
                .collect::<Vec<_>>();

            let mut bail = false;
            if let Some(prev_constants) = prev_constants {
                for (curr, prev) in constants.iter().zip(prev_constants.iter()) {
                    if curr < &0 && curr == prev {
                        // Value did not change by incrementing the last variable, so it'll always stay negative
                        bail = true;
                    } else if curr < &0 && prev >= &0 {
                        // Value was positive or zero before and is now negative, so it'll just get less by further
                        // incrementing the variable
                        bail = true;
                    }
                }
            }
            prev_constants = Some(constants.clone());

            if bail {
                let max_value = min_max_values
                    .get(&equation_system.variables[values.len() - 1])
                    .unwrap()
                    .1;

                values.pop();
                values.push(max_value + 1);
            } else {
                // Constants are only valid when positive and divisible by the common factor
                let constants_valid = constants
                    .iter()
                    .all(|constant| constant >= &0 && constant % equation_system.multiplier == 0);
                if constants_valid {
                    let presses = constants.iter().sum::<isize>() / equation_system.multiplier;
                    match min_presses {
                        Some(min) => min_presses = Some(min.min(presses)),
                        None => min_presses = Some(presses),
                    }
                }
            }

            while values.last().map_or(false, |value| {
                value
                    > &min_max_values
                        .get(&equation_system.variables[values.len() - 1])
                        .unwrap()
                        .1
            }) {
                values.pop();
                prev_constants = None;
            }

            if values.len() > 0 {
                let last_value = values.pop().unwrap();
                values.push(last_value + 1);
            }
        }

        min_presses.unwrap() as usize
    }
}

struct EquationSystem {
    equations: Vec<(Vec<isize>, Constant)>,
    coefficient_count: usize,
    fixed_rows: usize,
    variables: Vec<Variable>,
    multiplier: isize,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Variable(String);

#[derive(Clone, PartialEq, Debug)]
enum ConstantPart {
    Literal(isize),
    Variable((Variable, isize)),
}

impl ToString for ConstantPart {
    fn to_string(&self) -> String {
        match self {
            Self::Literal(x) => x.to_string(),
            Self::Variable((Variable(x), a)) => match a {
                1 => format!("{x}"),
                -1 => format!("-{x}"),
                a => format!("{a}{x}"),
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Constant(Vec<ConstantPart>);

impl ToString for Constant {
    fn to_string(&self) -> String {
        if self.0.len() == 0 {
            return "0".to_string();
        }

        let mut sum_str = String::new();
        for part in self.0.iter() {
            let mut str = part.to_string();
            if sum_str.is_empty() {
                sum_str.push_str(&str);
            } else if str.starts_with("-") {
                sum_str.push_str(" - ");
                str.remove(0);
                sum_str.push_str(&str);
            } else {
                if !sum_str.is_empty() {
                    sum_str.push_str(" + ");
                }
                sum_str.push_str(&str);
            }
        }
        sum_str
    }
}

impl std::ops::Add for Constant {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut combined_parts = vec![];
        combined_parts.extend(self.0);
        combined_parts.extend(rhs.0);

        let mut parts = vec![];

        let mut literal = 0;
        let mut i = 0;
        while i < combined_parts.len() {
            match combined_parts[i] {
                ConstantPart::Literal(a) => {
                    literal += a;
                    combined_parts.remove(i);
                }
                ConstantPart::Variable(_) => i += 1,
            }
        }
        if literal != 0 {
            parts.push(ConstantPart::Literal(literal));
        }

        while combined_parts.len() > 0 {
            let (variable, mut factor) = match combined_parts.remove(0) {
                ConstantPart::Variable((x, a)) => (x, a),
                ConstantPart::Literal(_) => unreachable!(),
            };

            let mut i = 0;
            while i < combined_parts.len() {
                match &combined_parts[i] {
                    ConstantPart::Variable((x, a)) if x == &variable => {
                        factor += a;
                        combined_parts.remove(i);
                    }
                    ConstantPart::Variable(_) => i += 1,
                    ConstantPart::Literal(_) => unreachable!(),
                }
            }

            if factor != 0 {
                parts.push(ConstantPart::Variable((variable, factor)))
            }
        }

        Self(parts)
    }
}

impl std::ops::Sub for Constant {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs * -1)
    }
}

impl std::ops::Mul<isize> for Constant {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(
            self.0
                .into_iter()
                .map(|part| match part {
                    ConstantPart::Literal(a) => ConstantPart::Literal(a * rhs),
                    ConstantPart::Variable((x, a)) => ConstantPart::Variable((x, a * rhs)),
                })
                .collect(),
        )
    }
}

impl Constant {
    fn is_zero(&self) -> bool {
        self.0.len() == 0
    }

    fn as_literal(&self) -> Option<isize> {
        match self.0.len() {
            0 => Some(0),
            1 => match self.0[0] {
                ConstantPart::Literal(a) => Some(a),
                ConstantPart::Variable(_) => None,
            },
            _ => None,
        }
    }

    fn resolve(&self, variable: &Variable, value: isize) -> Self {
        let mut new = self.clone();

        let mut i = 0;
        while i < self.0.len() {
            match &self.0[i] {
                ConstantPart::Variable((v, a)) if v == variable => {
                    new.0.remove(i);
                    return new + Constant::new_literal(a * value);
                }
                _ => i += 1,
            }
        }

        new
    }

    fn resolve_all(&self, variables: &[(&Variable, isize)]) -> Self {
        let mut new = self.clone();
        for (variable, value) in variables {
            new = new.resolve(variable, *value);
        }
        new
    }

    fn new_literal(a: isize) -> Self {
        Self(vec![ConstantPart::Literal(a)])
    }

    fn new_variable(x: Variable) -> Self {
        Self(vec![ConstantPart::Variable((x, 1))])
    }
}

impl From<&Machine> for EquationSystem {
    fn from(value: &Machine) -> Self {
        let mut equations = value
            .joltage
            .iter()
            .map(|x| (vec![], Constant::new_literal(*x as isize)))
            .collect::<Vec<_>>();

        for button in value.buttons.iter() {
            for (i, x) in button.iter().enumerate() {
                equations[i].0.push(*x as isize);
            }
        }

        let coefficient_count = equations[0].0.len();

        Self {
            equations,
            coefficient_count,
            fixed_rows: 0,
            variables: vec![],
            multiplier: 1,
        }
    }
}

impl std::fmt::Display for EquationSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = "=".repeat((self.equations[0].0.len() + 1) * 8);

        let mut rows = Vec::with_capacity(self.equations.len() + 2);
        rows.push(separator.clone());

        for (coefficients, constant) in self.equations.iter() {
            let mut s = String::new();

            for c in coefficients {
                s.push_str(&c.to_string());
                s.push('\t');
            }

            s.push_str("| ");
            s.push_str(&constant.to_string());

            rows.push(s);
        }

        rows.push(separator);
        write!(f, "{}", rows.join("\n"))
    }
}

impl EquationSystem {
    fn solve(&mut self) {
        while self.fixed_rows < self.equations.len() {
            self.fix_next_row();
        }

        self.add_variables();

        for i in (0..self.coefficient_count).rev() {
            self.eliminate(i);
        }

        // All coefficients shall be the same, so find the least common multiplier
        for index in 0..self.coefficient_count {
            self.multiplier = lcm(self.multiplier, self.coefficient(index, index));
        }

        // Update all equations
        for index in 0..self.coefficient_count {
            let factor = self.multiplier / self.coefficient(index, index);
            self.multiply(index, factor);
        }
    }

    fn fix_next_row(&mut self) {
        // Find the index of the first column that has non-zero coefficients
        let mut first_non_zero_col = 0;
        let mut row_index = 0;
        'col_loop: while first_non_zero_col < self.coefficient_count {
            for row in self.fixed_rows..self.equations.len() {
                if self.coefficient(row, first_non_zero_col) != 0 {
                    row_index = row;
                    break 'col_loop;
                }
            }
            first_non_zero_col += 1;
        }

        if first_non_zero_col == self.coefficient_count {
            // That means all coefficients are zero, so the constant must also be zero
            for row in self.fixed_rows..self.equations.len() {
                if !self.constant(row).is_zero() {
                    panic!("Row {row} has all zero coefficients but a non-zero constant.");
                }
            }

            // All remaining rows are zero and can be removed
            for _ in self.fixed_rows..self.equations.len() {
                self.equations.remove(self.fixed_rows);
            }
            return;
        }

        // Reorder rows so that the next fixed one has a non-zero coefficient.
        let equation = self.equations.remove(row_index);
        self.equations.insert(self.fixed_rows, equation);

        // Make sure the coefficient of the soon-to-be fixed row is positive.
        if self.coefficient(self.fixed_rows, first_non_zero_col) < 0 {
            self.multiply(self.fixed_rows, -1);
        }

        // Make all other row coefficients zero by adding and subtracting the
        // soon-to-be fixed row.
        for row in self.fixed_rows + 1..self.equations.len() {
            let change_coefficient = self.coefficient(row, first_non_zero_col);
            if change_coefficient == 0 {
                continue;
            }
            let fixed_coefficient = self.coefficient(self.fixed_rows, first_non_zero_col);

            // Bring both rows on the same factor
            self.multiply(row, fixed_coefficient);
            self.multiply(
                self.fixed_rows,
                if change_coefficient < 0 {
                    -change_coefficient
                } else {
                    change_coefficient
                },
            );

            // Make the coefficient in the lower row zero
            if change_coefficient < 0 {
                self.add(row, self.fixed_rows);
            } else {
                self.subtract(row, self.fixed_rows);
            }

            // Reduce the rows to avoid large numbers
            self.reduce(row);
            self.reduce(self.fixed_rows);
        }

        self.fixed_rows += 1;
    }

    fn add_variables(&mut self) {
        let mut var_counter = 1;
        for i in 0..self.coefficient_count {
            if i >= self.equations.len() || self.coefficient(i, i) == 0 {
                let mut coefficients = vec![0; self.coefficient_count];
                coefficients[i] = 1;

                let variable = Variable(format!("x{var_counter}"));
                self.variables.push(variable.clone());
                self.equations
                    .insert(i, (coefficients, Constant::new_variable(variable)));
                var_counter += 1;
            }
        }
    }

    fn eliminate(&mut self, index: usize) {
        for row in 0..index {
            let change_coefficient = self.coefficient(row, index);
            if change_coefficient == 0 {
                continue;
            }
            let fixed_coefficient = self.coefficient(index, index);

            // Bring both rows on the same factor
            self.multiply(row, fixed_coefficient);
            self.multiply(
                index,
                if change_coefficient < 0 {
                    -change_coefficient
                } else {
                    change_coefficient
                },
            );

            // Make the coefficient in the upper row zero
            if change_coefficient < 0 {
                self.add(row, index);
            } else {
                self.subtract(row, index);
            }

            // Reduce the rows to avoid large numbers
            self.reduce(row);
            self.reduce(index);
        }
    }

    fn reduce(&mut self, row: usize) {
        let mut divisor: Option<isize> = None;
        for coefficient in self.equations[row].0.iter() {
            let coefficient = *coefficient;
            if coefficient == 0 {
                continue;
            }
            divisor = match divisor {
                Some(d) => Some(gcd(d, coefficient)),
                None => Some(coefficient),
            }
        }

        let max_divisor = match divisor {
            Some(d) => d,
            None => return,
        };

        let constant = self.constant(row).clone();
        for divisor in (2..=max_divisor).rev() {
            let are_coefficients_divisible = self.equations[row].0.iter().all(|c| c % divisor == 0);
            let is_constant_divisible = constant.0.iter().all(|part| match part {
                ConstantPart::Literal(a) => a % divisor == 0,
                ConstantPart::Variable((_, a)) => a % divisor == 0,
            });
            if are_coefficients_divisible && is_constant_divisible {
                for col in 0..self.coefficient_count {
                    self.equations[row].0[col] = self.equations[row].0[col] / divisor;
                }
                self.equations[row].1 = Constant(
                    constant
                        .0
                        .into_iter()
                        .map(|part| match part {
                            ConstantPart::Literal(a) => ConstantPart::Literal(a / divisor),
                            ConstantPart::Variable((x, a)) => {
                                ConstantPart::Variable((x, a / divisor))
                            }
                        })
                        .collect(),
                );

                return;
            }
        }
    }

    fn coefficient(&self, row: usize, col: usize) -> isize {
        self.equations[row].0[col]
    }

    fn constant(&self, row: usize) -> &Constant {
        &self.equations[row].1
    }

    fn multiply(&mut self, row: usize, factor: isize) {
        if factor == 1 {
            return;
        }

        for col in 0..self.coefficient_count {
            self.equations[row].0[col] = self.equations[row].0[col] * factor;
        }
        self.equations[row].1 = self.equations[row].1.clone() * factor;
    }

    fn add(&mut self, row: usize, other: usize) {
        for col in 0..self.coefficient_count {
            self.equations[row].0[col] = self.equations[row].0[col] + self.equations[other].0[col];
        }
        self.equations[row].1 = self.equations[row].1.clone() + self.equations[other].1.clone();
    }

    fn subtract(&mut self, row: usize, other: usize) {
        for col in 0..self.coefficient_count {
            self.equations[row].0[col] = self.equations[row].0[col] - self.equations[other].0[col];
        }
        self.equations[row].1 = self.equations[row].1.clone() - self.equations[other].1.clone();
    }
}

fn solver() -> (String, String) {
    let machines = INPUT
        .lines()
        .map(|line| Machine::from(line))
        .collect::<Vec<_>>();

    let mut sum_lights = 0;
    let mut sum_joltage = 0;

    for machine in machines.iter() {
        sum_lights += machine.toggle_lights();
        sum_joltage += machine.set_joltage();
    }

    (sum_lights.to_string(), sum_joltage.to_string())
}
