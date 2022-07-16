pub mod tabbed_reader;

use tabbed_reader::TabbedReader;

fn main() {
    let mut reader = TabbedReader::new("main.mc");
    reader.get_next_token();
}
