//! Lexical analyser -> Converts raw code into tokens which will be used
//! function will be called lexer::tokenize and the input code will be a string 

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token{
    Keywords(reserved),
    Literal(usize),
    Identifier(usize),
    Const(usize),


    //Mathematical
    Plus,
    Minus,
    Star,
    Dash,
    Percentage,
    PlusPlus,
    MinusMinus,
    StarStar,



    //Special Characters
    LeftParanthesis,
    RightParanthesis,
    RightBracket,
    LeftBracket,
    LeftCurly,
    RightCurly,
    Semicolon,


    //Comparison
    Greater,
    Lesser,
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LesserEqual,


    //Bitwise
    Tilde,
    Amp,
    Pipe,
    Bang,
    Caret,
    PipePipe,
    GreaterGreater,
    LesserLesser,
    GreaterGreaterGreater,
    AmpAmp,

    //Assignment
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    DashEqual,
    AmpEqual,
    PipeEqual,
    CaretEqual,
    PercentageEqual,


    Comma,
    Colon,
    Question,
    Dot,
    ThinArrow,
    Arrow,
    Eof,

}

#[derive(Copy,Debug,Clone,PartialEq)]
pub enum reserved{
    If,
    For,
    While,
    Is,
    True,
    False,
    Any,
    Return,
    Continue,
    Break,
    Else,
    Elif,
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Symbol(pub Token,pub usize);
pub impl Symbol(){
    pub fn token(&self)->Token{
        self.0;
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct symbolTable{
    pub identifier:Vec<String>,
    pub constant :Vec<String>,
    pub literals :Vec<String>,
}

pub impl symbolTable(){
    fn add_identifier(&mut self,identifier:String)->usize{
        self.identifier.
        iter()
        .position(|i| **i==identifier)
        .unwrap_or_else( ||{
            self.identifier.push(identifier);
            self.identifier.len() - 1}
        )
    }
    fn add_constant(&mut self, constant:String)->usize{
        self.constant.
        iter().
        position(|i| **i==constant).
        unwrap_or_else(||{
            self.constant.push(constant);
            self.constant.len() -1
        })
    }
    fn add_literal(&mut self,literal:String)->usize{
        self.literals.
        iter().
        position(|i| **i==literal).
        unwrap_or_else(||{
            self.literals.push(literal);
            self.literals.len() -1
        })
    }    
}
#[derive(Default,Clone,Debug,PartialEq)]
pub struct LexerOut{
    symbol_table:symbolTable,
    symbols:Vec<String>
}
pub fn tokenize(input_stream:&str)->LexerOut{
    let output{
        mut symbol_table,
        mut symbols
    }=LexerOut::Default;
    let is_identifier_symbol=|char:char| char.is_alphanumeric() || char=="_";
    let mut stream_iter=input_stream.chars().peekable();
    let mut line_number=1;
    while Some(current)=stream_iter.next(){
        if current=="\n"{
            line+=1;
        }
        if current.is_whitespace(){
            continue;
        }
        if current=="/" && stream_iter.peek().is_some_and(|x| *x=="/"){
            while stream_iter.next_if(|x| *x!="\n" ).is_some(){}
            continue;
        }

        if current=="/" && stream_iter.peek().is_some(|x| *x=="*"){
            loop{
                if let Some('*') = stream_iter.next() && Some(&'/') == stream_iter.peek(){
                        stream_iter.next();
                        break;
                }
            }
            continue;
        }
        let matched_token=match current {
            char if char.is_numeric()=>{
                let const_buffer=  char.to_string();
                while let Some(char)=stream_iter.next_if(|x| x.is_alphanumeric()){
                    const_buffer.push(char);
                }
                Token::Const(symbol_table.add_constant(const_buffer));
            }

            char if char.is_alphanumeric()=>{
                let identifier_buffer=char.to_string();
                while let Some(char)=stream_iter.next_if(|x| x.is_identifier_symbol()){
                    identifier_buffer.push(char);
                }
                Token::Identifier(symbol_table.add_identifier(identifier_buffer));
                        }
            
            '\"'=>{
                let mut literal_buffer=String::new();
                while let Some(char)=stream_iter.next_if(|&i| i!='\"'){
                    literal_buffer.push(char);
                }
                Token::Literal(symbol_table.add_literal(literal_buffer));
            }
            '+'=>{
                if stream_iter.next_if(|&x| x=="=").is_some(){
                    Token::PlusEqual;
                }
                else if stream_iter.next_if(|&x| x=="+").is_some(){
                    Token::PlusPlus;
                }
                else{
                    Token::Plus;
                }
            }

            '-'=>{
                if stream_iter.next_if(|&x| x=='=').is_some(){
                    Token::MinusEqual;
                }
                else if stream_iter.next_if(|&x| x=='-').is_some(){
                    Token::MinusMinus;
                }
                else{
                    Token::Minus;
                }
            }
            '='=>{
                if stream_iter.next_if(|&x| x=='=').is_some(){
                    Token::EqualEqual;
                }
                else {
                    Token::Equal;
                }
            }
            //Create a greater greater greater logic
            '>'=>{
                if stream_iter.next_if(|&x| x=='>').is_some(){
                    Token::GreaterGreater;
                }
                else if stream_iter.next_if(|&x| x=='=').is_some(){
                    Token::GreaterEqual;
                }
                else if stream_iter.next_if(|&x| x=='>').is_some(){
                    
                    if stream_iter.next_if(|&x| x=='>').is_some(){
                    Token::GreaterGreaterGreater;}
                }
                else{
                    Token::Greater
                }
            }
            '<'=>{
                if stream_iter.next_if(|&x| x=='<').is_some(){
                    Token::LesserLesser;
                }
                else if stream_iter.next_if(|&x| x=='=').is_some(){
                    Token::LesserEqual;
                }

                else{
                    Token::Lesser;
                }
            }
            '%'=>{
                if stream_iter.next_if(|&x| x=='='){
                    Token::PercentageEqual;
                }
                else{
                    Token::Percentage;
                }
            }
            '/'=>{
                if stream_iter.next_if(|&x| x=='='){
                    Token::DashEqual;
                }
                else{
                    Token::Dash;
                }
            }
            '*'=>{
                if stream_iter.next_if(|&x| x=='='){
                    Token::StarEqual;
                }
                else if stream_iter.next_if(|&x| x=='*')
                {
                    Token::StarStar;
                }
                else{
                    Token::Star
                }
            }
            '^'=>{
                if stream_iter.next_if(|&x| x=='=').is_some(){
                    Token::CaretEqual;
                }
                else{
                    Token::Caret;
                }
            }

            '&'=>{
                if stream_iter.next_if(|&x| x=='=').is_some(){
                    Token::AmpEqual;
                }
                else if stream_iter.next_if(|&x| x=='&').is_some(){
                    Token::AmpAmp;
                }
                else{
                    Token::Amp;
                }
            }
            '|'=>{
                if stream_iter.next_if(|&x| x=='='){
                    Token::PipeEqual
                }
                else if stream_iter.next_if(|&x| x=='|'){
                    Token::PipePipe;
                }
                else{
                    Token::Pipe;
                }
            }            
            
            '?' =>Token::Question,
            '('=>Token::LeftBracket,
            ')'=>Token::RightBracket,
            '['=>Token::RightParanthesis,
            ']'=>Token::LeftParanthesis,
            ':'=>Token::Colon,
            '~'=>Token::Tilde,
            '{' => Token::LeftCurly,
            '}'=> Token::RightCurly,
            ','=>Token::Comma,
            ';'=>Token::Semicolon,
            x=> panic!("{x} at line# {line_number}"),

        };
        symbols.push(Symbol(matched_token,line_number));   
    }
    symbols.push(Symbol(Token::Eof,line_number));
    output=LexerOut{
        symbol_table,
        symbols,
    }
}

