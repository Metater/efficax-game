pub mod tabbed_reader;

use tabbed_reader::{TabbedReader, TabbedReaderToken};

fn main() {
    let mut reader = TabbedReader::new("main.mc");
    let mut token = reader.get_next_token();
    while token != TabbedReaderToken::EOF {
        println!("{:?}", token);
        token = reader.get_next_token();
    }
    print!("Done!");
}
