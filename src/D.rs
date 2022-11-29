use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() {

    let start = Instant::now();
    solve(read_file(&String::from("D.txt")));
    println!("Vyřešeno za {:?}", start.elapsed());
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
        let pocet_lesu = data[2];
        let pocet_lesu: usize = pocet_lesu.parse().unwrap();

        let mut graf: Vec<Node> = vec![];
        let mut j = 0;

        let mut map: HashMap<(usize, usize), usize> = HashMap::new();

        //        let mut speed = String::new();
        //        let start = Instant::now();

        while j != pocet_lesu {
            j += 1;

            //            println!(
            //                "Les čislo {}/{} (čas: {:?})",
            //                j,
            //                pocet_lesu,
            //                start.elapsed(),
            //            );
            //
            //            if j % 10_000 == 0 {
            //                speed += &format!("{:?} za {} lesů\n", start.elapsed(), j).to_string();
            //                fs::write(format!("./speed{}", i), &speed).unwrap();
            //            }

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

            map.insert((x, y), id);

            if map.contains_key(&(x, y + 1)) {
                let neighbourId = map.get(&(x, y + 1)).unwrap().to_owned();
                graf[id].neighbours.push(neighbourId);
                if !graf[neighbourId].neighbours.contains(&id) {
                    graf[neighbourId].neighbours.push(id);
                }
            }

            if map.contains_key(&(x, y - 1)) {
                let neighbourId = map.get(&(x, y - 1)).unwrap().to_owned();
                graf[id].neighbours.push(neighbourId);
                if !graf[neighbourId].neighbours.contains(&id) {
                    graf[neighbourId].neighbours.push(id);
                }
            }

            if map.contains_key(&(x + 1, y)) {
                let neighbourId = map.get(&(x + 1, y)).unwrap().to_owned();
                graf[id].neighbours.push(neighbourId);
                if !graf[neighbourId].neighbours.contains(&id) {
                    graf[neighbourId].neighbours.push(id);
                }
            }

            if map.contains_key(&(x - 1, y)) {
                let neighbourId = map.get(&(x - 1, y)).unwrap().to_owned();
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
                let mut queue: Vec<usize> = vec![node.id];
                let mut xs: Vec<usize> = vec![];
                let mut ys: Vec<usize> = vec![];

                while queue.len() != 0 {
                    let current = queue.remove(0);
                    //                    println!("{:?}", queue);
                    visited[current] = true;
                    let neighbours = graf[current].neighbours.to_vec();
                    for n in neighbours {
                        if !queue.contains(&n) && visited[n] == false {
                            queue.push(n);
                        }
                    }
                    xs.push(graf[current].x);
                    ys.push(graf[current].y);
                }

                let area = (xs.iter().max().unwrap() - xs.iter().min().unwrap() + 1)
                    * (ys.iter().max().unwrap() - ys.iter().min().unwrap() + 1);
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
    fs::write("D.out", out).unwrap();
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
