use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

/// Represents a date with day, month, and year.
#[derive(Debug, Serialize, Deserialize)]
pub struct GDate {
    pub day: u32,
    pub month: u32,
    pub year: u32,
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
    pub id: i32,
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub mother_id: Option<i32>,
    pub father_id: Option<i32>,
    pub dob: Option<GDate>,
    pub dod: Option<GDate>,
    pub info: Option<String>,
}

/// Represents an edge in the graph with a from_person_id and a to_person_id.
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub from_person_id: i32,
    pub to_person_id: i32,
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

    /// Returns a person from the `GraphDatabase`.
    /// If the person does not exist, it returns a `Person` with all fields set to `None`.
    pub fn get_person(&self, person_id: i32) -> &Person {
        self.persons.get(&person_id).unwrap()
    }

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

    //Given an ID find all children for each child of id get other parent != given id
    pub fn get_mates(&self, person_id: i32) -> Vec<&Person> {
        let mut mates = Vec::new();
        for edge in &self.edges {
            if edge.from_person_id == person_id {
                let child = self.persons.get(&edge.to_person_id).unwrap();
                let mother_id = child.mother_id.unwrap();
                let father_id = child.father_id.unwrap();
                if mother_id != person_id {
                    mates.push(self.persons.get(&mother_id).unwrap());
                }
                if father_id != person_id {
                    mates.push(self.persons.get(&father_id).unwrap());
                }
            }
        }
        mates
    }

    /// Renders a tree of the `GraphDatabase` starting from a person with a given id.
    pub fn render_family_tree(&self, person_id: i32) {
        let person = self.get_person(person_id);
        print!("+ {} {}", person.firstname.as_ref().unwrap_or(&"Unknown".to_string()), person.surname.as_ref().unwrap_or(&"Unknown".to_string()));
        println!("|");

        let mother_id = person.mother_id.unwrap_or(-1);
        let father_id = person.father_id.unwrap_or(-1);

        if mother_id != -1 {
            let mother = self.get_person(mother_id);
            print!("+- Mother: {} {}", mother.firstname.as_ref().unwrap_or(&"Unknown".to_string()), mother.surname.as_ref().unwrap_or(&"Unknown".to_string()));
        }

        if father_id != -1 {
            let father = self.get_person(father_id);
            println!("+- Father: {} {}", father.firstname.as_ref().unwrap_or(&"Unknown".to_string()), father.surname.as_ref().unwrap_or(&"Unknown".to_string()));
        }

        let mates = self.get_mates(person_id);
        if !mates.is_empty() {
            println!("|");
            println!("+- Spouses:");
            for mate in mates {
                println!("   +- {} {}", mate.firstname.as_ref().unwrap_or(&"Unknown".to_string()), mate.surname.as_ref().unwrap_or(&"Unknown".to_string()));
            }
        }

        let children = self.get_children(person_id);
        if !children.is_empty() {
            println!("|");
            println!("+- Children:");
            for child in children {
                println!("   +- {} {}", child.firstname.as_ref().unwrap_or(&"Unknown".to_string()), child.surname.as_ref().unwrap_or(&"Unknown".to_string()));
            }
        }
    }

}

/// Creates a new `Person`.
pub fn create_person(id: i32, firstname: Option<String>, surname: Option<String>, mother_id: Option<i32>, father_id: Option<i32>, dob: Option<GDate>, dod: Option<GDate>, info: Option<String>) -> Person {
    Person { id, firstname, surname, mother_id, father_id, dob, dod, info }
}