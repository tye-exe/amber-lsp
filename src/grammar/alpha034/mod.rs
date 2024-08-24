use chumsky::{error::Simple, Parser, Stream};
use heraclitus_compiler::prelude::*;
use lexer::{get_rules, Token};

pub mod expressions;
pub mod global;
pub mod lexer;
pub mod parser;
pub mod statements;

#[derive(PartialEq, Debug, Clone)]
pub enum InterpolatedText {
    Escape(Spanned<String>),
    Expression(Box<Spanned<Expression>>),
    Text(Spanned<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Block {
    Block(Vec<Spanned<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum FailureHandler {
    Propagate,
    Handle(Vec<Spanned<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum InterpolatedCommand {
    Escape(String),
    CommandOption(String),
    Expression(Box<Spanned<Expression>>),
    Text(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Number(Spanned<f32>),
    Boolean(Spanned<bool>),
    Text(Vec<Spanned<InterpolatedText>>),
    Parentheses(Box<Spanned<Expression>>),
    Var(Spanned<String>),
    Add(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Subtract(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Multiply(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Divide(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Modulo(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Neg(Box<Spanned<Expression>>),
    And(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Or(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Gt(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Ge(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Lt(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Le(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Eq(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Neq(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Not(Box<Spanned<Expression>>),
    Ternary(
        Box<Spanned<Expression>>,
        Box<Spanned<Expression>>,
        Box<Spanned<Expression>>,
    ),
    FunctionInvocation(
        Spanned<String>,
        Vec<Spanned<Expression>>,
        Option<Spanned<FailureHandler>>,
    ),
    Command(
        Vec<Spanned<InterpolatedCommand>>,
        Option<Spanned<FailureHandler>>,
    ),
    Array(Vec<Spanned<Expression>>),
    Range(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Null,
    Cast(Box<Spanned<Expression>>, Spanned<String>),
    Status,
    Nameof(Box<Spanned<Expression>>),
    Is(Box<Spanned<Expression>>, Spanned<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ImportContent {
    ImportAll,
    ImportSpecific(Vec<Spanned<String>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum FunctionArgument {
    Generic(Spanned<String>),
    Typed(Spanned<String>, Spanned<TypeAnnotation>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeAnnotation {
    Type(Spanned<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IfCondition {
    IfCondition(Box<Spanned<Expression>>, Spanned<Block>),
    InlineIfCondition(Box<Spanned<Expression>>, Box<Spanned<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ElseCondition {
    Else(Spanned<Block>),
    InlineElse(Box<Spanned<Statement>>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IfChainContent {
    IfCondition(Spanned<IfCondition>),
    Else(Spanned<ElseCondition>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IterLoopVars {
    Single(Spanned<String>),
    WithIndex(Spanned<String>, Spanned<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum CommandModifier {
    Unsafe,
    Silent,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Expression(Box<Spanned<Expression>>),
    VariableInit(Spanned<String>, Box<Spanned<Expression>>),
    VariableSet(Spanned<String>, Box<Spanned<Expression>>),
    IfCondition(Spanned<IfCondition>, Option<Spanned<ElseCondition>>),
    IfChain(Vec<Spanned<IfChainContent>>),
    ShorthandAdd(Spanned<String>, Box<Spanned<Expression>>),
    ShorthandSub(Spanned<String>, Box<Spanned<Expression>>),
    ShorthandMul(Spanned<String>, Box<Spanned<Expression>>),
    ShorthandDiv(Spanned<String>, Box<Spanned<Expression>>),
    ShorthandModulo(Spanned<String>, Box<Spanned<Expression>>),
    InfiniteLoop(Spanned<Block>),
    IterLoop(
        Spanned<IterLoopVars>,
        Box<Spanned<Expression>>,
        Spanned<Block>,
    ),
    Break,
    Continue,
    Return(Option<Box<Spanned<Expression>>>),
    Fail(Option<Box<Spanned<Expression>>>),
    Echo(Box<Spanned<Expression>>),
    CommandModifier(Spanned<CommandModifier>),
    Block(Spanned<Block>),
    Comment(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum GlobalStatement {
    Import(Spanned<ImportContent>, Spanned<String>),
    FunctionDefinition(
        Spanned<String>,
        Vec<Spanned<FunctionArgument>>,
        Option<Spanned<TypeAnnotation>>,
        Vec<Spanned<Statement>>,
    ),
    Main(Vec<Spanned<Statement>>),
    Statement(Spanned<Statement>),
}

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

pub type SpannedSemanticToken = Spanned<usize>;

pub struct AmberCompiler {
    lexer: Compiler,
    parser: Box<dyn Parser<Token, Vec<Spanned<GlobalStatement>>, Error = Simple<Token>>>,
}

impl AmberCompiler {
    pub fn new() -> Self {
        let lexer = Compiler::new("Amber", get_rules());

        let parser = global::global_statement_parser();

        AmberCompiler {
            lexer,
            parser: Box::new(parser),
        }
    }

    pub fn tokenize(&mut self, input: &str) -> Vec<Spanned<Token>> {
        self.lexer.load(input);

        // It should never fail
        self.lexer
            .tokenize()
            .unwrap()
            .iter()
            .map(|t| (Token(t.word.clone()), t.start..(t.start + t.word.len())))
            .collect()
    }

    pub fn parse(&mut self, input: &str) -> (Option<Vec<Spanned<GlobalStatement>>>, Vec<Simple<Token>>) {
        let tokens = self.tokenize(input);
        let len = input.chars().count();
        self.parser.parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()))
    }
}
