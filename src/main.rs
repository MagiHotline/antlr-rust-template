use antlr_rust::{InputStream, common_token_stream::CommonTokenStream};
use bin_words::parser::{binwordslexer::BinWordsLexer, binwordsparser::BinWordsParser};

fn main() {
    println!("Enter a string to parse:");

    // Get user input from stdin
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let input = InputStream::new(input_string.trim());

    // Create a TokenSource from the CharStream using the BinWords grammar
    let lexer = BinWordsLexer::new(input);

    // Obtain the tokens from the TokenSource as a TokenStream
    let tokens = CommonTokenStream::new(lexer);

    // Create a parser that parses the BinWords grammar
    let mut parser = BinWordsParser::new(tokens);

    // Execute the grammar from the 'main' nonterminal symbol
    let _tree = parser.main();
}
