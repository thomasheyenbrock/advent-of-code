const INPUT: &str = include_str!("input.txt");

struct Device {
    gates: Vec<Gate>,
}

impl Device {
    fn new() -> Self {
        let mut iter = INPUT.split("\n\n");
        let mut gates = iter
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut iter = line.split(": ");
                let output = iter.next().unwrap().to_string();
                let value = iter.next().unwrap().parse::<u8>().unwrap();
                Gate {
                    inputs: None,
                    value: Some(value),
                    output,
                }
            })
            .collect::<Vec<_>>();

        gates.extend(iter.next().unwrap().lines().map(|line| {
            let mut iter = line.split(" ");
            let input1 = iter.next().unwrap().to_string();
            let operation = match iter.next().unwrap() {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => unreachable!(),
            };
            let input2 = iter.next().unwrap().to_string();
            iter.next();
            let output = iter.next().unwrap().to_string();
            Gate {
                inputs: Some((input1, input2, operation)),
                value: None,
                output,
            }
        }));

        Self { gates }
    }

    fn is_solved(&self) -> bool {
        self.gates.iter().all(|gate| gate.value.is_some())
    }

    fn iterate(&mut self) {
        let gates = self.gates.clone();
        for gate in self.gates.iter_mut() {
            if gate.value.is_some() {
                continue;
            }

            let (input1, input2, operation) = gate.inputs.clone().unwrap();

            let input1 = match gates
                .iter()
                .find_map(|gate| (gate.output == input1).then_some(gate.value))
                .unwrap()
            {
                Some(val) => val,
                None => continue,
            };
            let input2 = match gates
                .iter()
                .find_map(|gate| (gate.output == input2).then_some(gate.value))
                .unwrap()
            {
                Some(val) => val,
                None => continue,
            };

            gate.value = Some(match operation {
                Operation::And => input1 & input2,
                Operation::Or => input1 | input2,
                Operation::Xor => input1 ^ input2,
            });
        }
    }

    fn solution(&self) -> u64 {
        let mut solution = 0;

        for i in 0.. {
            let output = format!("z{:0>2}", i);
            let gate = match self.gates.iter().find(|gate| gate.output == output) {
                Some(gate) => gate,
                None => break,
            };
            solution |= (gate.value.unwrap() as u64) << i;
        }

        solution
    }
}

#[derive(Clone)]
struct Gate {
    inputs: Option<(String, String, Operation)>,
    value: Option<u8>,
    output: String,
}

#[derive(Clone, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

fn switch(gates: &mut Vec<Gate>, a: &str, b: &str) {
    for gate in gates.iter_mut() {
        if gate.output == a {
            gate.output = b.to_string();
        } else if gate.output == b {
            gate.output = a.to_string();
        }
    }
}

fn main() {
    // PART 1
    let mut device = Device::new();
    while !device.is_solved() {
        device.iterate();
    }
    println!("{}", device.solution());

    // PART 2: Just reverse-engineer the calculator
    //
    // x00 XOR y00 -> z00
    // x00 AND y00 -> ktt
    //
    // x01 XOR y01 -> rvb
    //   ktt XOR rvb -> z01
    //   ktt AND rvb -> kmb
    // x01 AND y01 -> kgp
    //   kgp OR kmb -> rkn
    //
    // x02 XOR y02 -> ssq
    //   rkn XOR ssq -> z02
    //   rkn AND ssq -> vsc
    // x02 AND y02 -> kwm
    //   kwm OR vsc -> ntj
    //
    // x03 XOR y03 -> fbk
    //   ntj XOR fbk -> z03
    //   ntj AND fbk -> dps
    // x03 AND y03 -> jmr
    //   jmr OR dps -> mpf
    //
    // x04 XOR y04 -> jjc
    //   mpf XOR jjc -> z04
    //   mpf AND jjc -> gvt
    // x04 AND y04 -> csm
    //   csm OR gvt -> cgt
    //
    // x05 XOR y05 -> kdm
    //   cgt XOR kdm -> z05
    //   cgt AND kdm -> sch
    // x05 AND y05 -> fjd
    //   fjd OR sch -> ftg
    //
    // ...and so on

    let mut gates = Device::new()
        .gates
        .into_iter()
        .filter(|gate| gate.inputs.is_some())
        .collect::<Vec<_>>();

    // These were the switches that made the code below not panic
    switch(&mut gates, "djg", "z12");
    switch(&mut gates, "sbg", "z19");
    switch(&mut gates, "hjm", "mcq");
    switch(&mut gates, "dsd", "z37");

    // ...which makes the result:
    let mut result = vec!["djg", "z12", "sbg", "z19", "hjm", "mcq", "dsd", "z37"];
    result.sort();
    println!("{}", result.join(","));

    let z0_gate_index = gates
        .iter()
        .enumerate()
        .find_map(|(i, g)| {
            let inputs = g.inputs.as_ref().unwrap();
            (inputs.2 == Operation::Xor
                && ((inputs.0 == "x00" && inputs.1 == "y00")
                    || (inputs.0 == "y00" && inputs.1 == "x00")))
                .then_some(i)
        })
        .unwrap();
    gates.remove(z0_gate_index);

    // c = carry (because this gate determines if there is a carry to the next bit)
    let c_index = gates
        .iter()
        .enumerate()
        .find_map(|(i, g)| {
            let inputs = g.inputs.as_ref().unwrap();
            (inputs.2 == Operation::And
                && ((inputs.0 == "x00" && inputs.1 == "y00")
                    || (inputs.0 == "y00" && inputs.1 == "x00")))
                .then_some(i)
        })
        .unwrap();
    let mut c = gates.remove(c_index).output;

    for bit in 1..44 {
        let x = format!("x{:0>2}", bit);
        let y = format!("y{:0>2}", bit);
        let z = format!("z{:0>2}", bit);

        let a_index = match gates.iter().enumerate().find_map(|(i, g)| {
            let inputs = g.inputs.as_ref().unwrap();

            (inputs.2 == Operation::Xor
                && ((inputs.0 == x && inputs.1 == y) || (inputs.0 == y && inputs.1 == x)))
                .then_some(i)
        }) {
            Some(index) => index,
            None => panic!("cannot find a_index for bit {bit}"),
        };
        let a = gates.remove(a_index).output;

        let r_index = match gates.iter().enumerate().find_map(|(i, g)| {
            let inputs = g.inputs.as_ref().unwrap();

            (inputs.2 == Operation::Xor
                && ((inputs.0 == a && inputs.1 == c) || (inputs.0 == c && inputs.1 == a))
                && g.output == z)
                .then_some(i)
        }) {
            Some(index) => index,
            None => panic!("cannot find r_index for bit {bit} (a={a}, c={c}, z={z})"),
        };
        gates.remove(r_index);

        let b_index = match gates.iter().enumerate().find_map(|(i, g)| {
            let inputs = g.inputs.as_ref().unwrap();

            (inputs.2 == Operation::And
                && ((inputs.0 == a && inputs.1 == c) || (inputs.0 == c && inputs.1 == a)))
                .then_some(i)
        }) {
            Some(index) => index,
            None => panic!("cannot find b_index for bit {bit}"),
        };
        let b = gates.remove(b_index).output;

        let f_index = match gates.iter().enumerate().find_map(|(i, g)| {
            let inputs = g.inputs.as_ref().unwrap();

            (inputs.2 == Operation::And
                && ((inputs.0 == x && inputs.1 == y) || (inputs.0 == y && inputs.1 == x)))
                .then_some(i)
        }) {
            Some(index) => index,
            None => panic!("cannot find f_index for bit {bit}"),
        };
        let f = gates.remove(f_index).output;

        let c_index = match gates.iter().enumerate().find_map(|(i, g)| {
            let inputs = g.inputs.as_ref().unwrap();

            (inputs.2 == Operation::Or
                && ((inputs.0 == b && inputs.1 == f) || (inputs.0 == f && inputs.1 == b)))
                .then_some(i)
        }) {
            Some(index) => index,
            None => panic!("cannot find c_index for bit {bit}"),
        };
        c = gates.remove(c_index).output;
    }
}
