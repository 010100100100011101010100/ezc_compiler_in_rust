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
    
}