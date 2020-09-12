use std::net::TcpListener;

fn main() {
    print!("FOMOS GUI \n");
    print!("127.0.0.1:8080 \n");
    gui();
}

fn gui() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("GUI working");
    }
}
