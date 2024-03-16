use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct GDate {
    day: u32,
    month: u32,
    year: u32,
}

impl GDate {
    pub fn new(day: u32, month: u32, year: u32) -> Self {
        GDate { day, month, year }
    }

    pub fn display_date(&self) {
        println!("{}/{}/{}", self.day, self.month, self.year);
    }
}

// Define a struct to represent a Person in the graph
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    id: i32,
    firstname: Option<String>,
    surname: Option<String>,
    mother_id: Option<i32>,
    father_id: Option<i32>,
    dob: Option<GDate>,
    dod: Option<GDate>,
    info: Option<String>,
}

// Define a struct to represent an Edge in the graph
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    from_person_id: i32,
    to_person_id: i32,
}


// Define a struct to represent the Graph Database
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphDatabase {
    persons: HashMap<i32, Person>,
    edges: Vec<Edge>,
}

impl GraphDatabase {
    pub fn new() -> Self {
        GraphDatabase {
            persons: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.persons.insert(person.id, person);
    }

    pub fn add_edge(&mut self, from_person_id: i32, to_person_id: i32) {
        let edge = Edge { from_person_id, to_person_id };
        self.edges.push(edge);
    }

    pub fn print_graph(&self) {
        println!("Persons:");
        for person in self.persons.values() {
            println!("{:?}", person);
        }

        println!("Edges:");
        for edge in &self.edges {
            println!("{:?}", edge);
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let serialized = to_string(&self)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(file_path: &str) -> std::io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let deserialized: GraphDatabase = serde_json::from_reader(reader)?;
        Ok(deserialized)
    }
}


//fn creqate_person funciton
pub fn create_person(id: i32, firstname: Option<String>, surname: Option<String>, mother_id: Option<i32>, father_id: Option<i32>, dob: Option<GDate>, dod: Option<GDate>, info: Option<String>) -> Person {
    Person { id, firstname, surname, mother_id, father_id, dob, dod, info }
}
