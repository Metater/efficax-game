use std::fs;

pub struct TabbedReader {
    data: Vec<String>,
    line: usize,
    char: usize,
}

impl TabbedReader {
    pub fn new(file_path: &str) -> Self {
        let data = fs::read_to_string(format!("E:\\Projects\\efficax-game\\schema\\{}", file_path))
            .expect(format!("unable to read file: {}", file_path).as_str());
        TabbedReader {
            data: data.lines().map(|x| x.to_string()).collect(),
            line: 0,
            char: 0,
        }
    }

    pub fn get_next_token(&mut self) -> TabbedReaderToken {
        for (il, l) in self.data.iter().enumerate() {
            if self.line == il {
                for (ic, c) in l.chars().enumerate() {
                    println!("{}:{}:{}", il, ic, c);
                }
            }
            println!("{}", il);
        }

        TabbedReaderToken::EOF
    }
}

#[derive(Debug)]
pub enum TabbedReaderToken {
    EOF,
    EOL,
    LeadingTabs(usize),
    FollowingText(String),
}