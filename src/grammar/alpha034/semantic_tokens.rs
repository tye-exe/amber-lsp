use std::ops::Range;

use tower_lsp::lsp_types::SemanticTokenType;

use crate::grammar::SpannedSemanticToken;

use super::*;

pub const LEGEND_TYPE: [SemanticTokenType; 10] = [
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::COMMENT,
    SemanticTokenType::NUMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::TYPE,
    SemanticTokenType::MODIFIER,
];

fn hash_semantic_token_type(token_type: SemanticTokenType) -> usize {
    LEGEND_TYPE.iter().position(|x| *x == token_type).unwrap()
}

pub fn semantic_tokens_from_ast(
    ast: &Option<Vec<Spanned<GlobalStatement>>>,
) -> Vec<SpannedSemanticToken> {
    ast.as_ref().map_or(vec![], |ast| {
        let mut tokens = vec![];

        for (statement, span) in ast {
            match statement {
                GlobalStatement::Import(import_content, path) => {
                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::KEYWORD),
                        span.start..(span.start + 6),
                    ));

                    match import_content {
                        (ImportContent::ImportAll, span) => {
                            tokens.push((
                                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                                span.clone(),
                            ));
                        }
                        (ImportContent::ImportSpecific(vars), _) => {
                            vars.iter().for_each(|(_, span)| {
                                tokens.push((
                                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                                    span.clone(),
                                ));
                            })
                        }
                    }

                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::STRING),
                        path.1.clone(),
                    ));
                }
                GlobalStatement::FunctionDefinition((_, name_span), args, ty, body) => {
                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::KEYWORD),
                        span.start..(span.start + 3),
                    ));

                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::FUNCTION),
                        name_span.clone(),
                    ));

                    args.iter().for_each(|(arg, _)| match arg {
                        FunctionArgument::Typed((_, arg_span), (_, ty_span)) => {
                            tokens.push((
                                hash_semantic_token_type(SemanticTokenType::PARAMETER),
                                arg_span.clone(),
                            ));
                            tokens.push((
                                hash_semantic_token_type(SemanticTokenType::TYPE),
                                ty_span.clone(),
                            ));
                        }
                        FunctionArgument::Generic((_, arg_span)) => {
                            tokens.push((
                                hash_semantic_token_type(SemanticTokenType::PARAMETER),
                                arg_span.clone(),
                            ));
                        }
                    });

                    if let Some((_, ty_span)) = ty {
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::TYPE),
                            ty_span.clone(),
                        ));
                    }

                    tokens.extend(semantic_tokens_from_stmnts(body));
                }
                GlobalStatement::Main(stmnts) => {
                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::KEYWORD),
                        span.start..(span.start + 4),
                    ));
                    tokens.extend(semantic_tokens_from_stmnts(stmnts));
                }
                GlobalStatement::Statement(stmnt) => {
                    tokens.extend(semantic_tokens_from_stmnts(&vec![stmnt.clone()]));
                }
            }
        }

        tokens
    })
}

fn semantic_tokens_from_stmnts(stmnts: &Vec<Spanned<Statement>>) -> Vec<SpannedSemanticToken> {
    stmnts
        .iter()
        .flat_map(|(stmnt, span)| match stmnt {
            Statement::Block((block, _)) => match block {
                Block::Block(stmnts) => semantic_tokens_from_stmnts(stmnts),
            },
            Statement::Break => vec![(
                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                span.clone(),
            )],
            Statement::CommandModifier((_, modifier_span)) => vec![(
                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                modifier_span.clone(),
            )],
            Statement::Comment(_) => vec![(
                hash_semantic_token_type(SemanticTokenType::COMMENT),
                span.clone(),
            )],
            Statement::Continue => vec![(
                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                span.clone(),
            )],
            Statement::Echo(expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::KEYWORD),
                    span.start..(span.start + 4),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
            Statement::Expression(expr) => semantic_tokens_from_expr(expr),
            Statement::Fail(expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::KEYWORD),
                    span.start..(span.start + 4),
                )];

                if let Some(expr) = expr {
                    tokens.extend(semantic_tokens_from_expr(expr));
                }

                tokens
            }
            Statement::IfChain(if_chain) => {
                let mut tokens = vec![];

                if_chain
                    .iter()
                    .for_each(|(chain_cond, _)| match chain_cond {
                        IfChainContent::IfCondition((if_cond, span)) => {
                            tokens.push((
                                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                                span.start..(span.start + 2),
                            ));

                            match if_cond {
                                IfCondition::IfCondition(expr, block) => {
                                    tokens.extend(semantic_tokens_from_expr(expr));
                                    tokens.extend(semantic_tokens_from_stmnts(&vec![(
                                        Statement::Block(block.clone()),
                                        block.1.clone(),
                                    )]));
                                }
                                IfCondition::InlineIfCondition(expr, stmnt) => {
                                    tokens.extend(semantic_tokens_from_expr(expr));
                                    tokens
                                        .extend(semantic_tokens_from_stmnts(&vec![*stmnt.clone()]));
                                }
                            }
                        }
                        IfChainContent::Else((else_cond, span)) => {
                            tokens.push((
                                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                                span.start..(span.start + 4),
                            ));

                            match else_cond {
                                ElseCondition::Else(block) => {
                                    tokens.extend(semantic_tokens_from_stmnts(&vec![(
                                        Statement::Block(block.clone()),
                                        block.1.clone(),
                                    )]));
                                }
                                ElseCondition::InlineElse(stmnt) => {
                                    tokens
                                        .extend(semantic_tokens_from_stmnts(&vec![*stmnt.clone()]));
                                }
                            }
                        }
                    });

                tokens
            }
            Statement::IfCondition((if_cond, _), else_cond) => {
                let mut tokens: Vec<(usize, std::ops::Range<usize>)> = vec![(
                    hash_semantic_token_type(SemanticTokenType::KEYWORD),
                    span.start..(span.start + 2),
                )];

                match if_cond {
                    IfCondition::IfCondition(expr, block) => {
                        tokens.extend(semantic_tokens_from_expr(expr));
                        tokens.extend(semantic_tokens_from_stmnts(&vec![(
                            Statement::Block(block.clone()),
                            block.1.clone(),
                        )]));
                    }
                    IfCondition::InlineIfCondition(expr, stmnt) => {
                        tokens.extend(semantic_tokens_from_expr(expr));
                        tokens.extend(semantic_tokens_from_stmnts(&vec![*stmnt.clone()]));
                    }
                }

                if let Some((else_cond, else_span)) = else_cond {
                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::KEYWORD),
                        else_span.start..(else_span.start + 4),
                    ));

                    match else_cond {
                        ElseCondition::Else(block) => {
                            tokens.extend(semantic_tokens_from_stmnts(&vec![(
                                Statement::Block(block.clone()),
                                block.1.clone(),
                            )]));
                        }
                        ElseCondition::InlineElse(stmnt) => {
                            tokens.extend(semantic_tokens_from_stmnts(&vec![*stmnt.clone()]));
                        }
                    }
                }

                tokens
            }
            Statement::InfiniteLoop(block) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::KEYWORD),
                    span.start..(span.start + 4),
                )];

                tokens.extend(semantic_tokens_from_stmnts(&vec![(
                    Statement::Block(block.clone()),
                    block.1.clone(),
                )]));

                tokens
            }
            Statement::IterLoop((vars, _), expr, block) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::KEYWORD),
                    span.start..(span.start + 4),
                )];

                match vars {
                    IterLoopVars::Single((_, span)) => {
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::VARIABLE),
                            span.clone(),
                        ));
                    }
                    IterLoopVars::WithIndex((_, span1), (_, span2)) => {
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::VARIABLE),
                            span1.clone(),
                        ));
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::VARIABLE),
                            span2.clone(),
                        ));
                    }
                }

                tokens.extend(semantic_tokens_from_expr(expr));
                tokens.extend(semantic_tokens_from_stmnts(&vec![(
                    Statement::Block(block.clone()),
                    block.1.clone(),
                )]));

                tokens
            }
            Statement::Return(expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::KEYWORD),
                    span.start..(span.start + 6),
                )];

                if let Some(expr) = expr {
                    tokens.extend(semantic_tokens_from_expr(expr));
                }

                tokens
            }
            Statement::ShorthandAdd((_, var_span), expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                    var_span.clone(),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
            Statement::ShorthandDiv((_, var_span), expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                    var_span.clone(),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
            Statement::ShorthandMul((_, var_span), expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                    var_span.clone(),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
            Statement::ShorthandModulo((_, var_span), expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                    var_span.clone(),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
            Statement::ShorthandSub((_, var_span), expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                    var_span.clone(),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
            Statement::VariableInit((_, var_span), expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                    var_span.clone(),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
            Statement::VariableSet((_, var_span), expr) => {
                let mut tokens = vec![(
                    hash_semantic_token_type(SemanticTokenType::VARIABLE),
                    var_span.clone(),
                )];

                tokens.extend(semantic_tokens_from_expr(expr));

                tokens
            }
        })
        .collect()
}

fn semantic_tokens_from_expr((expr, span): &Spanned<Expression>) -> Vec<SpannedSemanticToken> {
    match expr {
        Expression::Add(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::And(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Array(elements) => elements
            .iter()
            .flat_map(|expr| semantic_tokens_from_expr(expr))
            .collect(),
        Expression::Boolean(_) => vec![(
            hash_semantic_token_type(SemanticTokenType::KEYWORD),
            span.clone(),
        )],
        Expression::Cast(expr, (_, ty_span)) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(expr));
            tokens.push((
                hash_semantic_token_type(SemanticTokenType::TYPE),
                ty_span.clone(),
            ));

            tokens
        }
        Expression::Command(cmd, failure_handler) => {
            let mut tokens = vec![];

            tokens.push((
                hash_semantic_token_type(SemanticTokenType::STRING),
                span.start..(span.start + 1),
            ));
            cmd.iter().for_each(|(inter_cmd, span)| match inter_cmd {
                InterpolatedCommand::Text(_) => {
                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::STRING),
                        span.clone(),
                    ));
                }
                InterpolatedCommand::Expression(expr) => {
                    tokens.extend(semantic_tokens_from_expr(expr));
                }
                InterpolatedCommand::CommandOption(_) => {
                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::KEYWORD),
                        span.clone(),
                    ));
                }
                InterpolatedCommand::Escape(_) => {
                    tokens.push((
                        hash_semantic_token_type(SemanticTokenType::KEYWORD),
                        span.clone(),
                    ));
                }
            });

            tokens.push((
                hash_semantic_token_type(SemanticTokenType::STRING),
                (span.end - 1)..span.end,
            ));

            if let Some((failure_handler, failure_span)) = failure_handler {
                match failure_handler {
                    FailureHandler::Handle(stmnts) => {
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::KEYWORD),
                            failure_span.start..(failure_span.start + 6),
                        ));

                        tokens.extend(semantic_tokens_from_stmnts(stmnts));
                    }
                    FailureHandler::Propagate => {
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::KEYWORD),
                            failure_span.clone(),
                        ));
                    }
                }
            }

            tokens
        }
        Expression::Divide(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Eq(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::FunctionInvocation((_, name_span), args, failure_handler) => {
            let mut tokens = vec![(
                hash_semantic_token_type(SemanticTokenType::FUNCTION),
                name_span.clone(),
            )];

            args.iter().for_each(|expr| {
                tokens.extend(semantic_tokens_from_expr(expr));
            });

            if let Some((failure_handler, failure_span)) = failure_handler {
                match failure_handler {
                    FailureHandler::Handle(stmnts) => {
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::KEYWORD),
                            failure_span.start..(failure_span.start + 6),
                        ));

                        tokens.extend(semantic_tokens_from_stmnts(stmnts));
                    }
                    FailureHandler::Propagate => {
                        tokens.push((
                            hash_semantic_token_type(SemanticTokenType::KEYWORD),
                            failure_span.clone(),
                        ));
                    }
                }
            }

            tokens
        }
        Expression::Ge(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Gt(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Is(lhs, (_, ty_span)) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.push((
                hash_semantic_token_type(SemanticTokenType::TYPE),
                ty_span.clone(),
            ));

            tokens
        }
        Expression::Le(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Lt(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Modulo(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Multiply(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Nameof(expr) => {
            let mut tokens = vec![];

            tokens.push((
                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                span.start..(span.start + 6),
            ));
            tokens.extend(semantic_tokens_from_expr(expr));

            tokens
        }
        Expression::Neg(expr) => {
            let mut tokens = vec![];

            tokens.push((
                hash_semantic_token_type(SemanticTokenType::OPERATOR),
                span.start..(span.start + 1),
            ));
            tokens.extend(semantic_tokens_from_expr(expr));

            tokens
        }
        Expression::Neq(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Not(expr) => {
            let mut tokens = vec![];

            tokens.push((
                hash_semantic_token_type(SemanticTokenType::KEYWORD),
                span.start..(span.start + 3),
            ));
            tokens.extend(semantic_tokens_from_expr(expr));

            tokens
        }
        Expression::Null => vec![(
            hash_semantic_token_type(SemanticTokenType::KEYWORD),
            span.clone(),
        )],
        Expression::Number(_) => vec![(
            hash_semantic_token_type(SemanticTokenType::NUMBER),
            span.clone(),
        )],
        Expression::Or(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Parentheses(expr) => semantic_tokens_from_expr(expr),
        Expression::Range(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Status => vec![(
            hash_semantic_token_type(SemanticTokenType::KEYWORD),
            span.clone(),
        )],
        Expression::Subtract(lhs, rhs) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(lhs));
            tokens.extend(semantic_tokens_from_expr(rhs));

            tokens
        }
        Expression::Ternary(cond, if_true, if_false) => {
            let mut tokens = vec![];

            tokens.extend(semantic_tokens_from_expr(cond));
            tokens.extend(semantic_tokens_from_expr(if_true));
            tokens.extend(semantic_tokens_from_expr(if_false));

            tokens
        }
        Expression::Text(inter_text) => {
            let mut tokens = vec![];

            tokens.push((
                hash_semantic_token_type(SemanticTokenType::STRING),
                span.start..(span.start + 1),
            ));
            tokens.extend(
                inter_text
                    .iter()
                    .flat_map(|(text, span)| match text {
                        &InterpolatedText::Text(_) => vec![(
                            hash_semantic_token_type(SemanticTokenType::STRING),
                            span.clone(),
                        )],
                        InterpolatedText::Expression(expr) => semantic_tokens_from_expr(expr),
                        InterpolatedText::Escape(_) => vec![(
                            hash_semantic_token_type(SemanticTokenType::STRING),
                            span.clone(),
                        )],
                    })
                    .collect::<Vec<(usize, Range<usize>)>>(),
            );

            tokens.push((
                hash_semantic_token_type(SemanticTokenType::STRING),
                (span.end - 1)..span.end,
            ));

            tokens
        }
        Expression::Var(_) => vec![(
            hash_semantic_token_type(SemanticTokenType::VARIABLE),
            span.clone(),
        )],
    }
}
