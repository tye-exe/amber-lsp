/// This module contains the grammar for the Amber language.
/// 
/// Precedence levels (Lower on the list means higher precedence):
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
        VariableGet(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String
        ),
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
            #[rust_sitter::leaf(text = "?")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = ":")] (),
            Box<Expression>
        ),
        #[rust_sitter::prec(11)]
        FunctionInvocation(
            #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
            String,
            #[rust_sitter::leaf(text = "(")] (),
            #[rust_sitter::delimited(
                #[rust_sitter::leaf(text = ",")]
                ()
            )]
            Vec<Expression>,
            #[rust_sitter::leaf(text = ")")] ()
        ),
        Command(
            #[rust_sitter::leaf(text = "$")] (),
            Vec<InterpolatedCommand>,
            #[rust_sitter::leaf(text = "$")] ()
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

    #[rust_sitter::language]
    #[derive(PartialEq, Debug)]
    pub enum Statement {
        Expression(Box<Expression>),
    }
}
