use std::io;
use dataframe::GraphDatabase;
use dataframe::Person;
use dataframe::GDate;
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};


mod dataframe;
mod virt;
mod serv;

/// This function is used to start the server in a separate thread so that it can be killed easily
// Atomic Bools are cool, having an inter threaded bool means its also reserved memory
fn start_server(stop_signal: Arc<AtomicBool>) {
//Spawn the thread, I might end up usiong alot of these
    thread::spawn(move || {
        while !stop_signal.load(Ordering::SeqCst) {
            println!("Server is running...");
            print!("IP: 127.0.0.1:8080");
            serv::main();
            thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("Server has been stopped.");
    });
}

fn main() {
    //
    let stop_signal = Arc::new(AtomicBool::new(false));

    //Master Loop
    loop {
        let mut usercmmnd = String::new();
        io::stdin().read_line(&mut usercmmnd).expect("failed to read");

        // Trim the newline character and any leading/trailing whitespace
        let usercmmnd = usercmmnd.trim();

        let mut selected_db = "";

        //Match is just rust equivalent of a swithc statement, can list all commands
        match usercmmnd {
            "start server" => {
                start_server(stop_signal.clone());
            },
            "kill server" => {
                stop_signal.store(true, Ordering::SeqCst);
                 // Exit the loop and end the program
            },
            "confirm exit" => {
                print!("Bye! Have a Good Time!");
                break;
            },



            _ => println!("Unknown command"),
        }


    }
}