use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() {
    println!("Zadejte zadání: ");

    let start = Instant::now();
    solve(read_file(&String::from("G.txt")));
    println!("Vyřešeno za {:?}", start.elapsed());
}

pub fn is_between(low: isize, mid: isize, high: isize) -> bool {
    low <= mid && mid <= high
}

pub fn solve(input: String) {
    let splitted: Vec<&str> = input.split("\n").collect();

    let problems = splitted.first().unwrap();
    println!("{}", problems);
    let problems: i32 = problems.parse().unwrap();
    let mut i = 0;
    let mut line_pointer = 1;
    let mut out = String::new();

    while problems != i {
        println!("Řeším {}. z {} problémů...", i + 1, problems);

        let data: Vec<&str> = splitted[line_pointer].split(" ").collect();
        println!("{:?}", data);
        let pocet_bakterii = data[0];
        let pocet_bakterii: isize = pocet_bakterii.parse().unwrap();
        let time = data[1];
        let time: isize = time.parse().unwrap();

        let mut bakterie: Vec<(isize, isize)> = vec![];

        let mut j = 0;
        while j != pocet_bakterii {
            j += 1;

            line_pointer += 1;
            let pos: Vec<isize> = splitted[line_pointer].split(" ").map(|e| {
                let r: isize = e.parse().unwrap();
                r
            }).collect();

            bakterie.push((pos[0], pos[1]));
        }

        let max_area = pocet_bakterii * (2 * time + 1) * (2 * time + 1);
        let mut subtract = 0;
        let mut valued: Vec<((isize, isize), (isize, isize))> = vec![];

        let mut searched: Vec<((isize, isize), (isize, isize))> = vec![];

        for b in &bakterie {
            println!("Bakterie {:?}", b);
            for n in &bakterie {
                if b == n {
                    continue;
                }

                if searched.contains(&(*b, *n)) || searched.contains(&(*n, *b)) {
                    continue;
                }

                println!("| Pár s {:?}", n);
                let dx = n.0 - b.0;
                let dy = n.1 - b.1;

                let w = 2 * time - dx.abs() + 1;
                let h = 2 * time - dy.abs() + 1;
                println!("  | Výška a šířka: {}, {}", w, h);

                if w < 0 || h < 0 {
                    searched.push((*b, *n));
                    continue;
                }

                let mut area = w * h;

                let mut xfactor = time;
                if dx < 0 {
                    xfactor = -time;
                }

                let mut yfactor = time;
                if dy < 0 {
                    yfactor = -time;
                }

                let mut e1 = (b.0 + xfactor, n.1 - yfactor);
                let mut e2 = (n.0 - xfactor, b.1 + yfactor);

                if e1.0 > e2.0 {
                    (e1, e2) = (e2, e1);
                }

                if e1.1 > e2.1 {
                    (e1.1, e2.1) = (e2.1, e1.1);
                }

                println!("  | Okraje: {:?}, {:?}", e1, e2);
                println!("  | Test na překrytí:");
                for v in &valued {
                    let between_x0 = is_between(v.0.0, e1.0, v.1.0);
                    let between_x1 = is_between(v.0.0, e2.0, v.1.0);
                    let between_y0 = is_between(v.0.1, e1.1, v.1.1);
                    let between_y1 = is_between(v.0.1, e2.1, v.1.1);

                    if !((between_x0 || between_x1) && (between_y0 || between_y1)) {
                        continue;
                    }

                    let o1: (isize, isize);
                    let o2: (isize, isize);

                    if between_x0 {
                        o1 = (e1.0, v.0.1);
                    } else {
                        o1 = (e2.0, v.0.1);
                    }

                    if between_y0 {
                        o2 = (v.1.0, e1.1);
                    } else {
                        o2 = (v.1.0, e2.1);
                    }

                    println!("    | Okraje jsou: {:?}, {:?}", o1, o2);

                    area -= (o1.0 - o2.0).abs() * (o1.1 - o2.0).abs();
                }


                subtract += area;
                valued.push((e1, e2));
                searched.push((*b, *n));
            }
        }

        let area = max_area - subtract;

        println!("{:?}", bakterie);
        println!("area: {}, max area: {}", area, max_area);
        line_pointer += 1;
        out += &format!("{:?}\n", area);
        i += 1;
        println!("Vyřešeno {} z {} problémů...", i, problems);
    }
    fs::write("G.out", out).unwrap();
}

pub fn read_line() -> String {
    let mut val = String::new();
    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");
    val
}

pub fn read_file(file: &String) -> String {
    fs::read_to_string(file).expect("Failed to read the file")
}

pub fn write_file(file: &String, content: &String) {
    fs::write(file, content).unwrap();
}
