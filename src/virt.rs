//Creating a looped process that allows us to run commands in the terminal
use std::io;



fn main() {

    // let mut graph_db = GraphDatabase::new();
    //
    // // Adding persons to the graph
    // let child = dataframe::create_person(1, Some("John".to_string()), Some("Doe".to_string()), Option::from(3), Option::from(2), Option::from(crate::dataframe::GDate::new(1, 12, 2003)), None, None);
    // let child1 = dataframe::create_person(4, Some("marc".to_string()), Some("Doe".to_string()), Option::from(3), Option::from(2), Option::from(crate::dataframe::GDate::new(1, 12, 2003)), None, None);
    //
    // let child2 = dataframe::create_person(5, Some("lasrjk".to_string()), Some("andi".to_string()), Option::from(3), None, Option::from(crate::dataframe::GDate::new(1, 12, 2003)), None, None);
    // let person2 = dataframe::create_person(2, Some("Father".to_string()), Some("Doe".to_string()), None, None, Option::from(crate::dataframe::GDate::new(1, 12, 2000)), None, None);
    // let person3 = dataframe::create_person(3, Some("Mother".to_string()), Some("Doe".to_string()), None, None, Option::from(crate::dataframe::GDate::new(1, 12, 2000)), None, None);
    //
    // graph_db.add_person(child);
    // graph_db.add_person(child1);
    // graph_db.add_person(child2);
    // graph_db.add_person(person2);
    // graph_db.add_person(person3);
    //
    // // Adding edges between persons
    // graph_db.add_edge(2, 1);
    // graph_db.add_edge(3, 1);
    // graph_db.add_edge(3, 4);
    // graph_db.add_edge(3, 5);
    // graph_db.add_edge(2, 4);


    // println!("{:?}", graph_db.find_spouse(3));



    // Printing the graph
    // graph_db.print_graph();
    // graph_db.render_family_tree(2);

    // Save the graph to a file
    // let file_path = "graph_db.json";
    // if let Err(e) = graph_db.save_to_file(file_path) {
    //     eprintln!("Failed to save graph database: {}", e);
    // }

    // Load the graph from the file
    // let loaded_graph_db = GraphDatabase::load_from_file(file_path).unwrap();
    // loaded_graph_db.print_graph();

    // println!("{:?}", loaded_graph_db.get_children(1))

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");


    }
}

