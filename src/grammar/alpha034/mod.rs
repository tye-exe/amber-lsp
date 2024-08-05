use chumsky::{error::Simple, Parser};

pub mod expressions;
pub mod statements;
pub mod global;

#[derive(PartialEq, Debug, Clone)]
pub enum InterpolatedText {
    Escape(String),
    Expression(Box<Expression>),
    Text(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Block {
    Block(Vec<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum FailureHandler {
    Propagate,
    Handle(Vec<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum InterpolatedCommand {
    Escape(String),
    CommandOption(String),
    Expression(Box<Expression>),
    Text(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Number(f32),
    Boolean(bool),
    Text(Vec<InterpolatedText>),
    Parentheses(Box<Expression>),
    Var(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Ge(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Le(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    Neq(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    FunctionInvocation(String, Vec<Expression>, Option<FailureHandler>),
    Command(Vec<InterpolatedCommand>, Option<FailureHandler>),
    Array(Vec<Expression>),
    Range(Box<Expression>, Box<Expression>),
    Null,
    Cast(Box<Expression>, String),
    Status,
    Nameof(Box<Expression>),
    Is(Box<Expression>, String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ImportContent {
    ImportAll,
    ImportSpecific(Vec<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum FunctionArgument {
    Generic(String),
    Typed(String, TypeAnnotation),
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeAnnotation {
    Type(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IfCondition {
    IfCondition(Box<Expression>, Block),
    InlineIfCondition(Box<Expression>, Box<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ElseCondition {
    Else(Block),
    InlineElse(Box<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IfChainContent {
    IfCondition(IfCondition),
    Else(ElseCondition),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IterLoopVars {
    Single(String),
    WithIndex(String, String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum CommandModifier {
    Unsafe,
    Silent,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Expression(Box<Expression>),
    VariableInit(String, Box<Expression>),
    VariableSet(String, Box<Expression>),
    IfCondition(IfCondition, Option<ElseCondition>),
    IfChain(Vec<IfChainContent>),
    ShorthandAdd(String, Box<Expression>),
    ShorthandSub(String, Box<Expression>),
    ShorthandMul(String, Box<Expression>),
    ShorthandDiv(String, Box<Expression>),
    ShorthandModulo(String, Box<Expression>),
    InfiniteLoop(Block),
    IterLoop(IterLoopVars, Box<Expression>, Block),
    Break,
    Continue,
    Return(Option<Box<Expression>>),
    Fail(Option<Box<Expression>>),
    Echo(Box<Expression>),
    CommandModifier(CommandModifier),
    Block(Block),
    Comment(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum GlobalStatement {
    Import(ImportContent, String),
    FunctionDefinition(String, Vec<FunctionArgument>, Option<TypeAnnotation>, Vec<Statement>),
    Main(Vec<Statement>),
    Statement(Vec<Statement>),
}

pub fn parse() -> impl Parser<char, GlobalStatement, Error = Simple<char>> {
    global::global_statement_parser()
}
