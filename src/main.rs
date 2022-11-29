use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() {
    println!("Zadejte zadání: ");

    let start = Instant::now();
    solve(read_file(&String::from("E.txt")));
    println!("Vyřešeno za {:?}", start.elapsed());
}

pub fn deeper(layer: &usize, vyznamy: &Vec<Vec<usize>>, max: &usize, used: &usize) -> usize {
    let mut moznosti: usize = 0;

    for vyznam in &vyznamy[*layer] {
        if vyznam + used > *max {
            continue;
        }

        if vyznam + used == *max {
            if layer + 1 == vyznamy.len() {
                moznosti += 1;
                continue;
            }
        }

        if layer + 1 == vyznamy.len() {
            continue;
        }

        moznosti += deeper(&(layer + 1), vyznamy, max, &(used + vyznam));
    }
    return moznosti;
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
        println!("Vyřešeno {} z {} problémů...", i, problems);

        let data: Vec<&str> = splitted[line_pointer].split(" ").collect();
        println!("{:?}", data);
        let pocet_vyznamu = data[0];
        let pocet_vyznamu: usize = pocet_vyznamu.parse().unwrap();
        let max_jednicek = data[1];
        let max_jednicek: usize = max_jednicek.parse().unwrap();

        let mut vyznamy: Vec<Vec<usize>> = vec![];

        //        let mut speed = String::new();
        //        let start = Instant::now();
        let mut j = 0;

        while j != pocet_vyznamu {
            j += 1;

            line_pointer += 2;
            let mozne_vyznamy: Vec<&str> = splitted[line_pointer].split(" ").collect();

            let mozne_vyznamy: Vec<usize> = mozne_vyznamy
                .iter()
                .map(|e| {
                    let e: usize = e.parse().unwrap();
                    return format!("{:b}", e).matches("1").count();
                })
                .filter(|&e| e <= max_jednicek)
                .collect();

            vyznamy.push(mozne_vyznamy);
        }

        if vyznamy.iter().any(|e| e.is_empty()) {
            out += "0\n";
            line_pointer += 1;
            println!("Vyřešeno {} z {} problémů...", i, problems);
            i += 1;
            continue;
        }

        let mut map: HashMap<usize, usize> = HashMap::from([(0, 1)]);
        let mut layer = 0;

        for vyznam in &vyznamy {
            //println!("=====");
            //println!("map: {:?}", map);
            let mut new_map: HashMap<usize, usize> = HashMap::new();

            for v in vyznam {
                let test: Vec<(usize, usize)> = map.iter().map(|e| (*e.0 + v, *e.1)).collect();
                let test: HashMap<usize, usize> = HashMap::from_iter(test.into_iter());
                //println!("| test for {}: {:?}", v, test);

                for (k, v) in test {
                    if new_map.contains_key(&k) {
                        *new_map.get_mut(&k).unwrap() += v;
                    } else {
                        new_map.insert(k, v);
                    }
                }
            }

            //println!("new_map: {:?}", new_map);

            let copy = vyznamy.clone();

            if layer + 1 != vyznamy.len() {
                let max: usize = (&copy[(layer + 1)..])
                    .into_iter()
                    .map(|e| e.into_iter().max().unwrap())
                    .sum();
                let min: usize = (&copy[(layer + 1)..])
                    .into_iter()
                    .map(|e| e.into_iter().max().unwrap())
                    .sum();

                new_map = new_map
                    .into_iter()
                    .filter(|&e| e.0 + max >= max_jednicek || e.0 + min <= max_jednicek)
                    .collect();
            }

            map = new_map
                .into_iter()
                .filter(|&e| e.0 <= max_jednicek).collect();

            layer += 1;
        }

        let moznosti: usize = map[&max_jednicek];

        line_pointer += 1;
        println!("Vyřešeno {} z {} problémů...", i, problems);
        out += &format!("{}\n", moznosti);
        i += 1;
    }
    fs::write("E.out", out).unwrap();
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
