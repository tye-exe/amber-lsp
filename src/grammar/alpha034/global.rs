use chumsky::prelude::*;

use crate::T;

use super::{
    lexer::Token, parser::ident, statements::statement_parser, FunctionArgument, GlobalStatement,
    ImportContent, Spanned, Statement,
};

pub fn import_parser() -> impl Parser<Token, Spanned<GlobalStatement>, Error = Simple<Token>> {
    let import_all_parser = just(T!["*"]).map_with_span(|_, span| (ImportContent::ImportAll, span));

    let import_specific_parser = just(T!["{"])
        .ignore_then(
            ident("variable".to_string())
                .recover_with(skip_parser(
                    none_of([T!["}"], T!['"']]).map(|_| "".to_string()),
                ))
                .map_with_span(|name, span| (name, span))
                .separated_by(just(T![","]).recover_with(skip_parser(
                    none_of([T!["}"], T!['"']]).rewind().map(|_| T![","]),
                ))),
        )
        .then_ignore(
            just(T!["}"]).recover_with(skip_parser(none_of([T!['"']]).or_not().map(|_| T!["}"]))),
        )
        .map_with_span(|vars, span| (ImportContent::ImportSpecific(vars), span));

    let path_parser = just(T!['"'])
        .ignore_then(
            filter(|c: &Token| *c != T!['"'])
                .repeated()
                .map_with_span(|name, span| (name.iter().cloned().collect(), span)),
        )
        .then_ignore(
            just(T!['"']).recover_with(skip_parser(
                none_of([T!['"']])
                    .repeated()
                    .then(just(T!['"']))
                    .or_not()
                    .map(|_| T!['"']),
            )),
        );

    just(T!["pub"])
        .or_not()
        .map_with_span(|is_pub, span| (is_pub.is_some(), span))
        .then(just(T!["import"]).map_with_span(|_, span| ("import".to_string(), span)))
        .then(
            import_all_parser
                .or(import_specific_parser)
                .recover_with(skip_parser(
                    none_of([T!['"'], T!["from"]])
                        .or_not()
                        .map_with_span(|_, span| (ImportContent::ImportAll, span)),
                )),
        )
        .then(
            just(T!["from"])
                .recover_with(skip_parser(none_of([T!['"']]).or_not().map(|_| T!["from"])))
                .map_with_span(|_, span| ("from".to_string(), span)),
        )
        .then(
            path_parser.recover_with(skip_parser(
                any()
                    .or_not()
                    .map_with_span(|_, span| ("".to_string(), span)),
            )),
        )
        .map_with_span(|((((is_pub, imp), vars), from), path), span| {
            (GlobalStatement::Import(is_pub, imp, vars, from, path), span)
        })
}

fn type_parser() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> {
    just(T![':']).ignore_then(
        ident("type".to_string())
            .or(just(T!["["])
                .ignore_then(ident("type".to_string()))
                .then_ignore(just(T!["]"]))
                .map(|ty| format!("[{}]", ty)))
            .map_with_span(|name, span| (name, span)),
    )
}

pub fn function_parser() -> impl Parser<Token, Spanned<GlobalStatement>, Error = Simple<Token>> {
    let generic_arg_parser = ident("argument".to_string())
        .map_with_span(|name, span| (FunctionArgument::Generic((name, span.clone())), span));

    let typed_arg_parser = ident("argument".to_string())
        .map_with_span(|name, span| (name, span))
        .then(type_parser())
        .map_with_span(|(name, ty), span| (FunctionArgument::Typed(name, ty), span));

    let arg_parser = typed_arg_parser.or(generic_arg_parser);

    let args_parser = arg_parser
        .recover_with(skip_parser(
            none_of([T![')'], T!['{']]).map_with_span(|_, span| (FunctionArgument::Error, span)),
        ))
        .separated_by(just(T![","]).recover_with(skip_parser(
            none_of([T![')'], T!['{']]).rewind().map(|_| T![","]),
        )))
        .delimited_by(
            just(T!['(']),
            just(T![')']).recover_with(skip_parser(
                none_of([T!['{'], T!['}']]).or_not().map(|_| T![')']),
            )),
        );

    let ret_parser = type_parser().or_not().then(
        just(T!["{"])
            .ignore_then(
                statement_parser()
                    .recover_with(skip_parser(
                        none_of([T!['}']]).map_with_span(|_, span| (Statement::Error, span)),
                    ))
                    .repeated(),
            )
            .then_ignore(just(T!["}"]).recover_with(skip_parser(any().or_not().map(|_| T!["}"])))),
    );

    just(T!["pub"])
        .or_not()
        .map_with_span(|is_pub, span| (is_pub.is_some(), span))
        .then(just(T!["fun"]).map_with_span(|_, span| ("fun".to_string(), span)))
        .then(
            ident("function".to_string())
                .map_err(|err| Simple::custom(err.span(), "Expected function name"))
                .recover_with(skip_parser(any().or_not().map(|_| "".to_string())))
                .map_with_span(|name, span| (name, span)),
        )
        .then(args_parser.recover_with(skip_parser(
            none_of([T!["{"], T!["}"]]).or_not().map(|_| vec![]),
        )))
        .then(ret_parser.recover_with(skip_parser(any().or_not().map(|_| (None, vec![])))))
        .map_with_span(|((((is_pub, fun), name), args), (ty, body)), span| {
            (
                GlobalStatement::FunctionDefinition(is_pub, fun, name, args, ty, body),
                span,
            )
        })
}

pub fn main_parser() -> impl Parser<Token, Spanned<GlobalStatement>, Error = Simple<Token>> {
    just(T!["main"])
        .ignore_then(
            just(T!["{"])
                .recover_with(skip_parser(
                    any()
                        .repeated()
                        .then(just(T!['{']))
                        .or_not()
                        .map(|_| T!["{"]),
                ))
                .ignore_then(
                    statement_parser()
                        .recover_with(skip_parser(
                            none_of([T!['}']]).map_with_span(|_, span| (Statement::Error, span)),
                        ))
                        .repeated(),
                )
                .then_ignore(
                    just(T!["}"]).recover_with(skip_parser(any().or_not().map(|_| T!["}"]))),
                ),
        )
        .map_with_span(|body, span| (GlobalStatement::Main(body), span))
}

pub fn global_statement_parser(
) -> impl Parser<Token, Vec<Spanned<GlobalStatement>>, Error = Simple<Token>> {
    let statement =
        statement_parser().map(|stmnt| (GlobalStatement::Statement(stmnt.clone()), stmnt.1));

    import_parser()
        .or(function_parser())
        .or(main_parser())
        .or(statement)
        .then_ignore(just(T![';']).or_not())
        .recover_with(skip_then_retry_until([]))
        .repeated()
        .then_ignore(end())
}
