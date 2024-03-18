use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

/// Represents a date with day, month, and year.
#[derive(Debug, Serialize, Deserialize)]
pub struct GDate {
    day: u32,
    month: u32,
    year: u32,
}

impl GDate {
    /// Creates a new `GDate`.
    pub fn new(day: u32, month: u32, year: u32) -> Self {
        GDate { day, month, year }
    }

    /// Prints the date in the format day/month/year.
    pub fn display_date(&self) {
        println!("{}/{}/{}", self.day, self.month, self.year);
    }
}

/// Represents a person with an id, first name, surname, mother's id, father's id, date of birth, date of death, and additional info.
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    id: i32,
    pub(crate) firstname: Option<String>,
    surname: Option<String>,
    mother_id: Option<i32>,
    father_id: Option<i32>,
    dob: Option<GDate>,
    dod: Option<GDate>,
    info: Option<String>,
}

/// Represents an edge in the graph with a from_person_id and a to_person_id.
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    from_person_id: i32,
    to_person_id: i32,
}

/// Represents a graph database with persons and edges.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphDatabase {
    persons: HashMap<i32, Person>,
    edges: Vec<Edge>,
}

impl GraphDatabase {
    /// Creates a new `GraphDatabase`.
    pub fn new() -> Self {
        GraphDatabase {
            persons: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Adds a person to the `GraphDatabase` and returns a mutable reference to `self`.
    pub fn add_person(&mut self, person: Person) -> &mut Self {
        self.persons.insert(person.id, person);
        self
    }

    /// Adds an edge to the `GraphDatabase` and returns a mutable reference to `self`.
    pub fn add_edge(&mut self, from_person_id: i32, to_person_id: i32) -> &mut Self {
        let edge = Edge { from_person_id, to_person_id };
        self.edges.push(edge);
        self
    }

    /// Prints all persons and edges in the `GraphDatabase`.
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

    /// Searches for a person in the `GraphDatabase` by id, first name, or surname.
    // pub fn search_person(&self, search: &str) -> Option<(i32, Option<String>, Option<String>, Option<GDate>, Option<GDate>, Option<String>)> {
    //     for person in self.persons.values() {
    //         if person.id.to_string().contains(search) || person.firstname.as_ref().unwrap().contains(search) || person.surname.as_ref().unwrap().contains(search) {
    //             return Some((person.id, person.firstname.clone(), person.surname.clone(), person.dob.clone(), person.dod.clone(), person.info.clone()));
    //         }
    //     }
    //     None
    // }


    /// Returns all children of a person in the `GraphDatabase`.
    pub fn get_children(&self, person_id: i32) -> Vec<&Person> {
        let mut children = Vec::new();
        for edge in &self.edges {
            if edge.from_person_id == person_id {
                let child = self.persons.get(&edge.to_person_id).unwrap();
                children.push(child);
            }
        }
        children
    }

    /// Saves the `GraphDatabase` to a file.
    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let serialized = to_string(&self)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Loads a `GraphDatabase` from a file.
    pub fn load_from_file(file_path: &str) -> std::io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let deserialized: GraphDatabase = serde_json::from_reader(reader)?;
        Ok(deserialized)
    }
}

/// Creates a new `Person`.
pub fn create_person(id: i32, firstname: Option<String>, surname: Option<String>, mother_id: Option<i32>, father_id: Option<i32>, dob: Option<GDate>, dod: Option<GDate>, info: Option<String>) -> Person {
    Person { id, firstname, surname, mother_id, father_id, dob, dod, info }
}