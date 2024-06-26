use std::env;
use std::fs::File;
use std::io::Read;
use wikitext_table_parser::parser::{Event, WikitextTableParser};
use wikitext_table_parser::tokenizer::{
    get_all_cell_text_special_tokens, get_all_table_special_tokens, Tokenizer,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();

    // Attempt to open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error opening the file.");
            return;
        }
    };

    // Read the contents of the file into a String
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content) {
        eprintln!("Error reading the file into a string.");
        return;
    }
    let table_tokenizer = Tokenizer::build(get_all_table_special_tokens());
    let cell_tokenizer = Tokenizer::build(get_all_cell_text_special_tokens());
    let wikitext_table_parser =
        WikitextTableParser::new(table_tokenizer, cell_tokenizer, &content, true);
    for event in wikitext_table_parser {
        match event {
            Event::TableStart {} => {
                println!("Table START!");
            }
            Event::TableStyle { text: table_style } => {
                println!("table style{:?}#", table_style);
            }
            Event::TableCaption { text } => {
                println!("table name{:?}#", text);
            }
            Event::RowStyle { text: row_style } => {
                println!("----- {:?} -----", row_style);
            }
            Event::ColStart { cell_type } =>{
                print!("{:?} ",cell_type);
            }
            Event::ColStyle { text: col_style } => {
                print!("style: {:?} -> ", col_style);
            }
            Event::ColEnd { text } => {
                println!("data: {:?}", text);
            }
            Event::TableEnd {} => {
                println!("Table END!");
            }
            _ => {}
        }
    }
}
