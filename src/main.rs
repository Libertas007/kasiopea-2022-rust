use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() {
    println!("Zadejte zadání: ");
    let input = read_line();
    let input = input.trim();

    if input == String::from("D") {
        let start = Instant::now();
        solve(read_file(&String::from("D.txt")));
        println!("Vyřešeno za {:?}", start.elapsed());
    } else {
        println!("Nenalezeno");
    }
}

#[derive(Clone)]
struct Node {
    x: usize,
    y: usize,
    neighbours: Vec<usize>,
    id: usize,
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
        let vyska = data[0];
        let vyska: usize = vyska.parse().unwrap();
        let sirka = data[1];
        let sirka: usize = sirka.parse().unwrap();
        let pocet_lesu = data[2];
        let pocet_lesu: usize = pocet_lesu.parse().unwrap();

        let mut graf: Vec<Node> = vec![];
        let mut j = 0;

        let mut table: Vec<Vec<usize>> = vec![vec![0; sirka]; vyska];

        let mut speed = String::new();
        let startIndex = line_pointer + 1;

        let start = Instant::now();
        while j != pocet_lesu {
            j += 1;

            println!(
                "Les čislo {}/{} (čas: {:?})",
                j,
                pocet_lesu,
                start.elapsed(),
            );

            if j % 10_000 == 0 {
                speed += &format!("{:?} za {} lesů\n", start.elapsed(), j).to_string();
                fs::write(format!("./speed{}", i), &speed).unwrap();
            }

            line_pointer += 1;
            let data: Vec<&str> = splitted[line_pointer].split(" ").collect();
            let x: usize = data[0].parse().unwrap();
            let y: usize = data[1].parse().unwrap();
            let id = graf.len();

            graf.push(Node {
                x,
                y,
                neighbours: vec![],
                id,
            });

            table[x][y] = id;

            if table[x][y + 1] != 0 {
                let neighbourId = graf
                    .iter()
                    .find(|node| node.x == x && node.y == y + 1)
                    .unwrap()
                    .id;
                graf[id].neighbours.push(neighbourId);
                if !graf[neighbourId].neighbours.contains(&id) {
                    graf[neighbourId].neighbours.push(id);
                }
            }

            if table[x][y - 1] != 0 {
                let neighbourId = graf
                    .iter()
                    .find(|node| node.x == x && node.y == y - 1)
                    .unwrap()
                    .id;
                graf[id].neighbours.push(neighbourId);
                if !graf[neighbourId].neighbours.contains(&id) {
                    graf[neighbourId].neighbours.push(id);
                }
            }

            if table[x + 1][y] != 0 {
                let neighbourId = graf
                    .iter()
                    .find(|node| node.x == x + 1 && node.y == y)
                    .unwrap()
                    .id;
                graf[id].neighbours.push(neighbourId);
                if !graf[neighbourId].neighbours.contains(&id) {
                    graf[neighbourId].neighbours.push(id);
                }
            }

            if table[x - 1][y] != 0 {
                let neighbourId = graf
                    .iter()
                    .find(|node| node.x == x - 1 && node.y == y)
                    .unwrap()
                    .id;
                graf[id].neighbours.push(neighbourId);
                if !graf[neighbourId].neighbours.contains(&id) {
                    graf[neighbourId].neighbours.push(id);
                }
            }
        }

        let mut total_area: usize = 0;

        let mut visited = vec![false; pocet_lesu];

        for node in &graf {
            if visited[node.id] == false {
                let mut inComponent: Vec<usize> = vec![];
                let mut queue: Vec<usize> = vec![node.id];
                while queue.len() != 0 {
                    let current = queue.remove(0);
                    println!("{:?}", queue);
                    io::stdout().flush().unwrap();
                    visited[current] = true;
                    let mut neighbours = graf[current].neighbours.to_vec();
                    for n in neighbours {
                        if !queue.contains(&n) && visited[n] == false {
                            queue.push(n);
                        }
                    }
                    inComponent.push(current);
                }

                let test = inComponent.to_vec();
                let xs = test.into_iter().map(|e| graf[e].x).into_iter();
                let test = inComponent.to_vec();
                let ys = test.into_iter().map(|e| graf[e].y).into_iter();

                let test = xs.clone();
                let xmax = test.max().unwrap();
                let xmin = xs.min().unwrap();
                let test = ys.clone();
                let ymax = test.max().unwrap();
                let ymin = ys.min().unwrap();

                let area = ((xmax - xmin + 1) * (ymax - ymin + 1)) as usize;
                total_area += area;
            }
        }

        out += &total_area.to_string();
        out += "\n";

        line_pointer += 1;
        println!("Vyřešeno {} z {} problémů...", i, problems);
        println!("{} cells", total_area);
        i += 1;
    }
    fs::write("C.out", out).unwrap();
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
