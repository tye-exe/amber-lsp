use crate::grammar::GlobalStatement;

use super::{
    super::JumpDefinitionResult, Block, ElseCondition, IfChainContent,
    IfCondition, ImportContent, IterLoopVars, Spanned, Statement,
};

pub fn get_definition(
    ast: &Vec<Spanned<GlobalStatement>>,
    (ident, ident_offset): (&str, usize),
    is_public: bool,
) -> JumpDefinitionResult {
    let mut def_span = JumpDefinitionResult::None;

    for (global, span) in ast.iter() {
        if span.start > ident_offset {
            break;
        }

        match global {
            GlobalStatement::FunctionDefinition(is_pub, _, (name, name_span), _, _, _) => {
                if is_public && !is_pub.0 {
                    continue;
                }

                if ident == name && name_span.start <= ident_offset && ident_offset <= name_span.end
                {
                    def_span = JumpDefinitionResult::InFile(span.clone());
                }
            }
            GlobalStatement::Main(body) => {
                if span.end < ident_offset || is_public {
                    continue;
                }

                for stmnt in body.iter() {
                    if stmnt.1.start > ident_offset {
                        break;
                    }

                    let main_def_span = get_definition_stmnt(stmnt, (ident, ident_offset));

                    if main_def_span != JumpDefinitionResult::None {
                        def_span = main_def_span;
                    }
                }
            }
            GlobalStatement::Import(is_pub, _, (import_content, _), _, (path, _)) => {
                if is_public && !is_pub.0 {
                    continue;
                }

                match import_content {
                    ImportContent::ImportSpecific(ident_list) => {
                        let imported_ident = ident_list.iter().find(|(ident, _)| ident == ident);

                        if let Some((_, span)) = imported_ident {
                            if span.start == ident_offset {
                                def_span = JumpDefinitionResult::OpenFile(path.clone());
                            } else {
                                def_span = JumpDefinitionResult::InFile(span.clone());
                            }
                        }
                    }
                    ImportContent::ImportAll => {
                        def_span = JumpDefinitionResult::OpenFile(path.clone());
                    }
                }
            }
            GlobalStatement::Statement(stmnt) => {
                if span.end < ident_offset || is_public {
                    continue;
                }

                def_span = get_definition_stmnt(stmnt, (ident, ident_offset));
            }
        }
    }

    def_span
}

pub fn get_definition_stmnt(
    (stmnt, span): &Spanned<Statement>,
    (ident, ident_offset): (&str, usize),
) -> JumpDefinitionResult {
    match stmnt {
        Statement::Block(block) => get_definition_block(block, (ident, ident_offset)),
        Statement::IfChain(if_chain) => {
            for (if_chain_content, span) in if_chain.iter() {
                if span.end < ident_offset {
                    continue;
                }

                match if_chain_content {
                    IfChainContent::IfCondition((condition, _)) => match condition {
                        IfCondition::IfCondition(_, block) => {
                            return get_definition_block(block, (ident, ident_offset))
                        }
                        IfCondition::InlineIfCondition(_, boxed_stmnt) => {
                            return get_definition_stmnt(boxed_stmnt, (ident, ident_offset))
                        }
                        _ => return JumpDefinitionResult::None,
                    },
                    IfChainContent::Else((else_cond, _)) => match else_cond {
                        ElseCondition::Else(block) => {
                            return get_definition_block(block, (ident, ident_offset))
                        }
                        ElseCondition::InlineElse(stmnt) => {
                            return get_definition_stmnt(stmnt, (ident, ident_offset))
                        }
                    },
                }
            }

            JumpDefinitionResult::None
        }
        Statement::IfCondition(if_cond, else_cond) => {
            if if_cond.1.start < ident_offset && ident_offset < if_cond.1.end {
                match &if_cond.0 {
                    IfCondition::IfCondition(_, block) => {
                        return get_definition_block(block, (ident, ident_offset))
                    }
                    IfCondition::InlineIfCondition(_, boxed_stmnt) => {
                        return get_definition_stmnt(boxed_stmnt, (ident, ident_offset))
                    }
                    _ => return JumpDefinitionResult::None,
                }
            } else if let Some(else_cond) = else_cond {
                if else_cond.1.end < ident_offset {
                    return JumpDefinitionResult::None;
                }

                match &else_cond.0 {
                    ElseCondition::Else(block) => {
                        return get_definition_block(block, (ident, ident_offset))
                    }
                    ElseCondition::InlineElse(stmnt) => {
                        return get_definition_stmnt(stmnt, (ident, ident_offset))
                    }
                }
            }

            JumpDefinitionResult::None
        }
        Statement::InfiniteLoop(block) => get_definition_block(block, (ident, ident_offset)),
        Statement::IterLoop(vars, _, block) => {
            let in_loop_vars = match &vars.0 {
                IterLoopVars::WithIndex((var1, var1_span), (var2, var2_span)) => {
                    if var1 == ident && var1_span.start <= ident_offset {
                        return JumpDefinitionResult::InFile(vars.1.clone());
                    } else if var2 == ident && var2_span.start <= ident_offset {
                        return JumpDefinitionResult::InFile(vars.1.clone());
                    }

                    JumpDefinitionResult::None
                }
                IterLoopVars::Single((var, var_span)) => {
                    if var == ident && var_span.start <= ident_offset {
                        return JumpDefinitionResult::InFile(vars.1.clone());
                    }

                    JumpDefinitionResult::None
                }
                _ => JumpDefinitionResult::None,
            };

            let block_def_span = get_definition_block(block, (ident, ident_offset));

            if block_def_span != JumpDefinitionResult::None {
                return block_def_span;
            }

            in_loop_vars
        }
        Statement::VariableInit((var_name, _), _) => {
            if var_name == ident {
                return JumpDefinitionResult::InFile(span.clone());
            }

            JumpDefinitionResult::None
        }
        _ => JumpDefinitionResult::None,
    }
}

pub fn get_definition_block(
    (block, span): &Spanned<Block>,
    (ident, ident_offset): (&str, usize),
) -> JumpDefinitionResult {
    if span.end < ident_offset || ident_offset < span.start {
        return JumpDefinitionResult::None;
    }

    let mut def_span = JumpDefinitionResult::None;

    if let Block::Block(stmnt) = block {
        for stmnt in stmnt.iter() {
            if ident_offset < stmnt.1.start {
                break;
            }

            let block_def_span = get_definition_stmnt(stmnt, (ident, ident_offset));

            if block_def_span != JumpDefinitionResult::None {
                def_span = block_def_span;
            }
        }
    }

    def_span
}
