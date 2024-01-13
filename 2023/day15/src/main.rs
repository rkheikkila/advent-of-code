fn main() {
    let input = include_str!("../input.txt");
    let a1 = part1(input);
    println!("{}", a1);
    let a2 = part2(input);
    println!("{}", a2);
}

#[derive(PartialEq, Eq, Debug)]
struct Lens<'a> {
    label: &'a str,
    length: usize,
}

fn hash(input: &str) -> usize {
    let mut value = 0;
    for c in input.chars() {
        value += c as usize;
        value *= 17;
        value = value.rem_euclid(256);
    }
    return value;
}

fn part1(input: &str) -> usize {
    let sum: usize = input.split(",").map(|seq| hash(seq)).sum();
    return sum;
}

fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        let vec = Vec::new();
        boxes.push(vec);
    }

    for seq in input.split(",") {
        let label = seq
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>();
        let hash_value = hash(label.as_str());
        let bx = &mut boxes[hash_value];

        if seq.contains("=") {
            let (label, length) = seq.split_once("=").unwrap();
            let lens = Lens {
                label: label,
                length: length.parse::<usize>().unwrap(),
            };
            let index = bx.iter().position(|x| x.label == lens.label);
            match index {
                Some(i) => {
                    bx.remove(i);
                    bx.insert(i, lens);
                }
                None => {
                    bx.push(lens);
                }
            };
        } else {
            let label = seq.strip_suffix("-").unwrap();
            let index = bx.iter().position(|x| x.label == label);
            match index {
                Some(i) => {
                    bx.remove(i);
                }
                None => (),
            }
        }
    }

    let mut sum = 0;
    for (i, bx) in boxes.iter().enumerate() {
        for (j, lens) in bx.iter().enumerate() {
            let power = (1 + i) * (1 + j) * lens.length;
            sum += power;
        }
    }

    return sum;
}

#[test]
fn test1() {
    let result = hash("HASH");
    assert_eq!(result, 52);
}

#[test]
fn test2() {
    let result = part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
    assert_eq!(result, 1320);
}

#[test]
fn test3() {
    let result = part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
    assert_eq!(result, 145);
}
