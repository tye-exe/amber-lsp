pub use super::Spanned;
use super::{Grammar, LSPAnalysis, ParserResponse, Span};
use chumsky::{
    error::Rich,
    extra::Err,
    input::{Input, SpannedInput},
    span::SimpleSpan,
    Parser,
};
use heraclitus_compiler::prelude::*;
use lexer::{get_rules, Token};
use prelude::lexer::Lexer;
use semantic_tokens::semantic_tokens_from_ast;

pub mod expressions;
pub mod global;
pub mod lexer;
pub mod parser;
pub mod semantic_tokens;
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
    Error,
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
    Error,
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
    Error,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeAnnotation {
    Type(Spanned<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IfCondition {
    IfCondition(Box<Spanned<Expression>>, Spanned<Block>),
    InlineIfCondition(Box<Spanned<Expression>>, Box<Spanned<Statement>>),
    Error,
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
    Error,
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
    Error,
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

pub struct AmberCompiler {
    lexer: Lexer,
}

impl AmberCompiler {
    pub fn new() -> Self {
        let lexer = Lexer::new(get_rules());

        AmberCompiler { lexer }
    }

    #[inline]
    pub fn parser<'a>(&self) -> impl AmberParser<'a, Vec<Spanned<GlobalStatement>>> {
        global::global_statement_parser()
    }
}

impl LSPAnalysis for AmberCompiler {
    #[inline]
    fn tokenize(&self, input: &str) -> Vec<Spanned<Token>> {
        // It should never fail
        self.lexer
            .tokenize(input)
            .unwrap()
            .iter()
            .filter_map(|t| {
                if t.word == "\n" {
                    return None;
                }

                return Some((
                    Token(t.word.clone()),
                    SimpleSpan::new(t.start, t.start + t.word.chars().count()),
                ));
            })
            .collect()
    }

    fn parse<'a>(&self, tokens: &'a Vec<Spanned<Token>>) -> ParserResponse<'a> {
        let len = tokens.last().map(|t| t.1.end).unwrap_or(0);
        let parser_input = tokens.spanned(Span::new(len, len));

        let result = self.parser().parse(parser_input);

        let semantic_tokens = semantic_tokens_from_ast(result.output());

        let string_errors = result
            .errors()
            .into_iter()
            .map(|e| e.clone().map_token(|t| t.0))
            .collect();

        ParserResponse {
            ast: Grammar::Alpha034(result.into_output()),
            errors: string_errors,
            semantic_tokens,
        }
    }
}

pub type RichError<'src> = Err<Rich<'src, Token>>;
type AmberInput<'src> = SpannedInput<Token, Span, &'src [Spanned<Token>]>;

pub trait AmberParser<'src, Output>:
    Parser<'src, AmberInput<'src>, Output, RichError<'src>> + Clone + Sized + 'src
{
}

impl<'src, Output, T> AmberParser<'src, Output> for T where
    T: Parser<'src, AmberInput<'src>, Output, RichError<'src>> + Clone + Sized + 'src
{
}
