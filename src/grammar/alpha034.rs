/// This module contains the grammar for the Amber language version 0.3.4-alpha.
/// 
/// Precedence levels (Lower on the list means higher precedence - i.e. it will be evaluated first):
/// 1. Closure
/// 2. Assignment
/// 3. Range
/// 4. Or
/// 5. And
/// 6. Comparison
/// 7. Additive
/// 8. Multiplicative
/// 9. Cast
/// 10. Unary
/// 11. Call
#[rust_sitter::grammar("Amber_v0_3_4_alpha")]
pub mod grammar {
    #[derive(PartialEq, Debug)]
    pub enum Variable {
        Variable(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String
        ),
    }

    #[derive(PartialEq, Debug)]
    pub enum Expression {
        Number(
            #[rust_sitter::leaf(
                pattern = r"\-?\d+(\.\d+)?",
                transform = |v| v.parse::<f32>().unwrap()
            )]
            f32
        ),
        Boolean(Boolean),
        Text(
            #[rust_sitter::leaf(text = "\"")] (),
            Vec<InterpolatedText>,
            #[rust_sitter::leaf(text = "\"")] ()
        ),
        #[rust_sitter::prec(1)]
        Parentheses(
            #[rust_sitter::leaf(text = "(")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = ")")] ()
        ),
        VariableGet(Variable),
        #[rust_sitter::prec_left(7)]
        Add(
            Box<Expression>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(7)]
        Subtract(
            Box<Expression>,
            #[rust_sitter::leaf(text = "-")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(8)]
        Multiply(
            Box<Expression>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(8)]
        Divide(
            Box<Expression>,
            #[rust_sitter::leaf(text = "/")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(8)]
        Modulo(
            Box<Expression>,
            #[rust_sitter::leaf(text = "%")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_right(10)]
        Neg(
            #[rust_sitter::leaf(text = "-")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(5)]
        And(
            Box<Expression>,
            #[rust_sitter::leaf(text = "and")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(4)]
        Or(
            Box<Expression>,
            #[rust_sitter::leaf(text = "or")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(6)]
        Gt(
            Box<Expression>,
            #[rust_sitter::leaf(text = ">")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(6)]
        Ge(
            Box<Expression>,
            #[rust_sitter::leaf(text = ">=")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(6)]
        Lt(
            Box<Expression>,
            #[rust_sitter::leaf(text = "<")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(6)]
        Le(
            Box<Expression>,
            #[rust_sitter::leaf(text = "<=")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(6)]
        Eq(
            Box<Expression>,
            #[rust_sitter::leaf(text = "==")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_left(6)]
        Neq(
            Box<Expression>,
            #[rust_sitter::leaf(text = "!=")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_right(10)]
        Not(
            #[rust_sitter::leaf(text = "not")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_right(1)]
        Ternary(
            Box<Expression>,
            #[rust_sitter::leaf(text = "then")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "else")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec_right(11)]
        FunctionInvocation(
            Variable,
            #[rust_sitter::leaf(text = "(")] (),
            #[rust_sitter::delimited(
                #[rust_sitter::leaf(text = ",")]
                ()
            )]
            Vec<Expression>,
            #[rust_sitter::leaf(text = ")")] (),
            Option<FailureHandler>,
        ),
        #[rust_sitter::prec_right(1)]
        Command(
            #[rust_sitter::leaf(text = "$")] (),
            Vec<InterpolatedCommand>,
            #[rust_sitter::leaf(text = "$")] (),
            Option<FailureHandler>,
        ),
        Array(
            #[rust_sitter::leaf(text = "[")] (),
            #[rust_sitter::delimited(
                #[rust_sitter::leaf(text = ",")]
                ()
            )]
            Vec<Expression>,
            #[rust_sitter::leaf(text = "]")] ()
        ),
        #[rust_sitter::prec_left(3)]
        Range(
            Box<Expression>,
            #[rust_sitter::leaf(text = "..")] (),
            #[rust_sitter::leaf(text = "=")] Option<()>,
            Box<Expression>,
        ),
        Null(
            #[rust_sitter::leaf(text = "null")] (),
        ),
        #[rust_sitter::prec_left(9)]
        Cast(
            Box<Expression>,
            #[rust_sitter::leaf(text = "as")] (),
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
        ),
        Status(
            #[rust_sitter::leaf(text = "status")] (),
        ),
        #[rust_sitter::prec_right(8)]
        Nameof(
            #[rust_sitter::leaf(text = "nameof")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(8)]
        Is(
            Box<Expression>,
            #[rust_sitter::leaf(text = "is")] (),            
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
        )
    }

    #[derive(PartialEq, Debug)]
    pub enum InterpolatedCommand {
        #[rust_sitter::prec(3)]
        Escape(
            #[rust_sitter::leaf(text = "\\")] (),
            #[rust_sitter::leaf(pattern = ".", transform = |v| v.to_string())]
            String,
        ),
        CommandOption(
            #[rust_sitter::leaf(pattern = "-{1,2}")] (),
            #[rust_sitter::leaf(pattern = "[a-zA-Z][A-Za-z0-9-_]*", transform = |v| v.to_string())]
            String
        ),
        Expression(
            #[rust_sitter::leaf(text = "{")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
        Text(
            #[rust_sitter::leaf(pattern = r#"[^\\${-]+"#, transform = |v| v.to_string())]
            String,
        ),
    }

    #[derive(PartialEq, Debug)]
    pub enum InterpolatedText {
        #[rust_sitter::prec(3)]
        Escape(
            #[rust_sitter::leaf(text = "\\")] (),
            #[rust_sitter::leaf(pattern = ".", transform = |v| v.to_string())]
            String,
        ),
        Expression(
            #[rust_sitter::leaf(text = "{")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
        Text(
            #[rust_sitter::leaf(pattern = r#"[^\\"{]+"#, transform = |v| v.to_string())]
            String,
        ),
    }

    #[derive(PartialEq, Debug)]
    pub enum Boolean {
        #[rust_sitter::leaf(text = "true")]
        True,
        #[rust_sitter::leaf(text = "false")]
        False,
    }

    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }

    #[rust_sitter::extra]
    struct Comment {
        #[rust_sitter::leaf(pattern = r"\/\/[^\/].*")]
        _comment: (),
    }

    #[derive(PartialEq, Debug)]
    pub enum IfCondition {
        IfCondition(
            Box<Expression>,
            Block,
        ),
        InlineIfCondition(
            Box<Expression>,
            #[rust_sitter::leaf(text = ":")] (),
            Box<Statement>,
        ),
    }

    #[derive(PartialEq, Debug)]
    pub enum ElseCondition {
        Else(
            #[rust_sitter::leaf(text = "else")] (),
            Block,
        ),
        InlineElse(
            #[rust_sitter::leaf(text = "else")] (),
            #[rust_sitter::leaf(text = ":")] (),
            Box<Statement>,
        )
    }

    #[derive(PartialEq, Debug)]
    pub enum IfChainContent {
        IfCondition(IfCondition),
        Else(ElseCondition)
    }

    #[derive(PartialEq, Debug)]
    pub enum IterLoopVars {
        Single(Variable),
        WithIndex(
            Variable,
            #[rust_sitter::leaf(text = ",")] (),
            Variable,
        )
    }

    #[derive(PartialEq, Debug)]
    pub enum FunctionArgument {
        Generic(Variable),
        Typed(
            Variable,
            TypeAnnotation,
        )
    }

    #[derive(PartialEq, Debug)]
    pub enum TypeAnnotation {
        Type(
            #[rust_sitter::leaf(text = ":")] (),
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
        )
    }

    #[derive(PartialEq, Debug)]
    pub enum ImportContent {
        ImportAll(
            #[rust_sitter::leaf(text = "*")] (),
        ),
        ImportSpecific(
            #[rust_sitter::leaf(text = "{")] (),
            #[rust_sitter::delimited(
                #[rust_sitter::leaf(text = ",")]
                ()
            )]
            Vec<Variable>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
    }

    #[derive(PartialEq, Debug)]
    pub enum CommandModifier {
        Unsafe(
            #[rust_sitter::leaf(text = "unsafe")] (),
        ),
        Silent(
            #[rust_sitter::leaf(text = "silent")] (),
        ),
    }

    #[derive(PartialEq, Debug)]
    pub enum FailureHandler {
        Propagate(
            #[rust_sitter::leaf(text = "?")] (),
        ),
        #[rust_sitter::prec_right(1)]
        Handle(
            #[rust_sitter::leaf(text = "failed")] (),
            Block,
        )
    }

    #[derive(PartialEq, Debug)]
    pub enum StatementWithSemi {
        #[rust_sitter::prec_right(1)]
        Statement(
            Statement,
            #[rust_sitter::leaf(text = ";")] Option<()>,
        ),
    }

    #[derive(PartialEq, Debug)]
    pub enum Block {
        #[rust_sitter::prec_right(1)]
        Block (
            #[rust_sitter::leaf(text = "{")] (),
            Vec<StatementWithSemi>,
            #[rust_sitter::leaf(text = "}")] (),
        )
    }

    #[derive(PartialEq, Debug)]
    pub enum GlobalStatement {
        Import(
            #[rust_sitter::leaf(text = "import")] (),
            ImportContent,
            #[rust_sitter::leaf(pattern = r#""[^"]*""#, transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = ";")] Option<()>,
        ),
        FunctionDefinition(
            #[rust_sitter::leaf(text = "fun")] (),
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = "(")] (),
            #[rust_sitter::delimited(
                #[rust_sitter::leaf(text = ",")]
                ()
            )]
            Vec<FunctionArgument>,
            #[rust_sitter::leaf(text = ")")] (),
            Option<TypeAnnotation>,
            Block,
        ),
        Main(
            #[rust_sitter::leaf(text = "main")] (),
            Block,
        ),
        Statement(StatementWithSemi),
    }

    #[derive(PartialEq, Debug)]
    pub enum Statement {
        Expression(Box<Expression>),
        VariableInit(
            #[rust_sitter::leaf(text = "let")] (),
            Variable,
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
        VariableSet(
            Variable,
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_right(1)]
        IfCondition(
            #[rust_sitter::leaf(text = "if")] (),
            IfCondition,
            Option<ElseCondition>
        ),
        IfChain(
            #[rust_sitter::leaf(text = "if")] (),
            #[rust_sitter::leaf(text = "{")] (),
            Vec<IfChainContent>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
        ShorthandAdd(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = "+=")] (),
            Box<Expression>,
        ),
        ShorthandSub(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = "-=")] (),
            Box<Expression>,
        ),
        ShorthandMul(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = "*=")] (),
            Box<Expression>,
        ),
        ShorthandDiv(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = "/=")] (),
            Box<Expression>,
        ),
        ShorthandModulo(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = "%=")] (),
            Box<Expression>,
        ),
        InfiniteLoop(
            #[rust_sitter::leaf(text = "loop")] (),
            Block,
        ),
        IterLoop(
            #[rust_sitter::leaf(text = "loop")] (),
            IterLoopVars,
            #[rust_sitter::leaf(text = "in")] (),
            Box<Expression>,
            Block,
        ),
        Break(
            #[rust_sitter::leaf(text = "break")] (),
        ),
        Continue(
            #[rust_sitter::leaf(text = "continue")] (),
        ),
        #[rust_sitter::prec_right(1)]
        Return(
            #[rust_sitter::leaf(text = "return")] (),
            Option<Box<Expression>>,
        ),
        #[rust_sitter::prec_right(1)]
        Fail(
            #[rust_sitter::leaf(text = "fail")] (),
            Option<Box<Expression>>,
        ),
        #[rust_sitter::prec_right(1)]
        Echo(
            #[rust_sitter::leaf(text = "echo")] (),
            Box<Expression>,
        ),
        CommandModifier(CommandModifier),
        Block(Block)
    }

    #[rust_sitter::language]
    #[derive(PartialEq, Debug)]
    pub struct Language {
        #[rust_sitter::repeat(non_empty = true)]
        pub statements: Vec<GlobalStatement>,
        // Statement(
        //     #[rust_sitter::repeat(non_empty = true)]
        //     Vec<GlobalStatement>,
        // )
    }
}
