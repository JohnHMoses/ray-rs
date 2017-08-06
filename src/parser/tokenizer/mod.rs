pub mod token;
pub mod tests;
use std::char;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;


struct TokenizerState {
    token: token::Token
}

struct Tokenizer<'a> {
    /// input stream
    stream: Peekable<Chars<'a>>,
    /// last read token, if it exists
    un_get_token: Option<token::Token>,
}

impl<'a> Tokenizer<'a>  {
    pub fn new(stream: Peekable<Chars<'a>>) -> Tokenizer<'a> {
        Tokenizer { stream, un_get_token: None }
    }

    /// moves the iterator through whitespace and comments
    fn skip_white_space(& mut self) {
        // TODO: rewrite this which slightly more inspired iterator methods
        let mut stream = &mut self.stream;

        'outer: loop {
            match stream.next() {
                Some(current_char) if char::is_whitespace(current_char) => continue,
                Some('/') => {
                    match stream.next() {
                        // check for normal comments
                        Some('/') => {
                            // run stream to end of line
                            run_to_pattern(stream, '\n');
                        },
                        // check for multi-line comments //TODO: doesn't handle nested /* */ comments
                        Some('*') => { 
                            //  run the stream until we find terminating comment token "*/"
                           'inner: loop {
                                run_to_pattern(stream, '*');
                                if let Some('/') = stream.next() {
                                    // move the stream forward and return
                                    break 'outer;
                                }
                            }
                        },
                        // single slash follow by unexpected character or EOF
                        _ => unimplemented!(),
                    }
                },
                _ => break,
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = token::Token;
    fn next(& mut self) -> Option<token::Token> {

        // first check to see if there is an un_get_token, and use it if yes
        if let Some(token) = self.un_get_token {
            self.un_get_token = None;
            return Some(token);
        }

        // else, scan for the next token

        self.skip_white_space();

        None
    }

}

fn run_to_pattern(stream: &mut Peekable<Chars>, c: char) {
    while let Some(current_char) = stream.next() {
        if current_char == c {
            return;
        }
    }
}