use std::io::{BufReader, BufRead};
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    fn contains(&self, assignment: Assignment) -> bool {
        if self.start <= assignment.start && self.end >= assignment.end || self.start >= assignment.start && self.end <= assignment.end {
            return true;
        }
        return false;
    }

    fn collides(&self, assignment: Assignment) -> bool {
        if assignment.start <= self.start && self.start <= assignment.end || assignment.start <= self.end && self.end <= assignment.end {
            return true;
        } else if self.start <= assignment.start && assignment.start <= self.end || self.start <= assignment.end && assignment.end <= self.end {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy)]
struct AssignmentPair {
    elf_one: Assignment,
    elf_two: Assignment,
}

fn parse_data() -> Vec<AssignmentPair> {
    let filename = "src/rawdata.txt";
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut assignments: Vec<AssignmentPair> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors because I'm lazy
        let mut data = line.split(",");

        if data.clone().count() == 2 { // has to be cloned lest it be consumed
            let elf_one = data.next().unwrap();
            let elf_two = data.next().unwrap();
            let mut elf_one_data = elf_one.split("-");
            let mut elf_two_data = elf_two.split("-");

            let assignment_pair: AssignmentPair = AssignmentPair {
                elf_one: Assignment {
                    start: elf_one_data.next().unwrap().parse::<i32>().unwrap(),
                    end: elf_one_data.next().unwrap().parse::<i32>().unwrap(),
                },
                elf_two: Assignment {
                    start: elf_two_data.next().unwrap().parse::<i32>().unwrap(),
                    end: elf_two_data.next().unwrap().parse::<i32>().unwrap(),
                },
            };
            
            assignments.push(assignment_pair);
        }
    }

    return assignments;
}

fn main() {
    let data = parse_data();

    let mut contain_count: i32 = 0;
    for pair in data.clone() {
        if pair.elf_one.contains(pair.elf_two){
            contain_count += 1;
        }
    }

    println!("Overlap count: {}", contain_count);

    let mut collide_count: i32 = 0;
    for pair in data.clone() {
        if pair.elf_one.collides(pair.elf_two){
            collide_count += 1;
        }
    }

    println!("Collide count: {}", collide_count);
}
