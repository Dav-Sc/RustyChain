use dataframe::GraphDatabase;
use dataframe::Person;
use dataframe::GDate;

mod dataframe;

fn main() {
    let mut graph_db = GraphDatabase::new();

    // Adding persons to the graph
    let person1 = Person { id: 1, firstname: Some("John".to_string()), surname: Some("Doe".to_string()), mother_id: None, father_id: None, dob: Option::from(GDate::new(1, 12, 2003)), dod: None, info: None };
    // let person1 = dataframe::create_person(1, Some("John".to_string()), Some("Doe".to_string()), Option::from(3), Option::from(2), Option::from(crate::dataframe::GDate::new(1, 12, 2003)), None, None);
    //
    // let person2 = dataframe::create_person(2, Some("Father".to_string()), Some("Doe".to_string()), None, None, Option::from(crate::dataframe::GDate::new(1, 12, 2000)), None, None);
    // let person3 = dataframe::create_person(3, Some("Mother".to_string()), Some("Doe".to_string()), None, None, Option::from(crate::dataframe::GDate::new(1, 12, 2000)), None, None);
    //
    graph_db.add_person(person1).add_edge(1, 2);
    // graph_db.add_person(person2);
    // graph_db.add_person(person3);
    //
    // // Adding edges between persons
    // graph_db.add_edge(1, 2);
    // graph_db.add_edge(1, 3);



    // Printing the graph
    graph_db.print_graph();

    // Save the graph to a file
    // let file_path = "graph_db.json";
    // if let Err(e) = graph_db.save_to_file(file_path) {
    //     eprintln!("Failed to save graph database: {}", e);
    // }

    // Load the graph from the file
    // let loaded_graph_db = GraphDatabase::load_from_file(file_path).unwrap();
    // loaded_graph_db.print_graph();

    // println!("{:?}", loaded_graph_db.get_children(1))
}
