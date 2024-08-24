use chumsky::prelude::*;

use crate::T;

use super::{
    lexer::Token, parser::ident, statements::statement_parser, FunctionArgument, GlobalStatement,
    ImportContent, Span, Spanned, TypeAnnotation,
};

pub fn import_parser() -> impl Parser<Token, Spanned<GlobalStatement>, Error = Simple<Token>> {
    let import_all_parser = just(T!["*"]).map_with_span(|_, span| (ImportContent::ImportAll, span));

    let import_specific_parser = just(T!["{"])
        .ignore_then(
            ident()
                .map_with_span(|name, span| (name, span))
                .separated_by(just(T![","])),
        )
        .then_ignore(just(T!["}"]))
        .map_with_span(|vars, span| (ImportContent::ImportSpecific(vars), span));

    let path_parser = just(T!['"'])
        .ignore_then(
            filter(|c: &Token| *c != T!['"'])
                .repeated()
                .map_with_span(|name, span| (name.iter().cloned().collect(), span)),
        )
        .then_ignore(just(T!['"']));

    just(T!["import"])
        .ignore_then(import_all_parser.or(import_specific_parser))
        .then(path_parser)
        .map_with_span(|(vars, path), span| (GlobalStatement::Import(vars, path), span))
}

fn type_parser() -> impl Parser<Token, Spanned<TypeAnnotation>, Error = Simple<Token>> {
    just(T![':'])
        .ignore_then(ident().map_with_span(|name, span| (name, span)))
        .map_with_span(|name, span| (TypeAnnotation::Type(name), span))
}

pub fn function_parser() -> impl Parser<Token, Spanned<GlobalStatement>, Error = Simple<Token>> {
    let generic_arg_parser = ident()
        .map_with_span(|name, span: Span| (FunctionArgument::Generic((name, span.clone())), span));

    let typed_arg_parser = ident()
        .map_with_span(|name, span| (name, span))
        .then(type_parser())
        .map_with_span(|(name, ty), span| (FunctionArgument::Typed(name, ty), span));

    let arg_parser = typed_arg_parser.or(generic_arg_parser);

    let args_parser = arg_parser
        .separated_by(just(T![","]).recover_with(skip_then_retry_until([T![')']])))
        .delimited_by(
            just(T!['(']),
            just(T![')']).recover_with(skip_then_retry_until([T!['{'], T!['}'], T![']']])),
        );

    let ret_parser = type_parser().or_not().then(
        just(T!["{"])
            .ignore_then(statement_parser().repeated())
            .then_ignore(just(T!["}"]).recover_with(skip_then_retry_until([]))),
    );

    just(T!["fun"])
        .ignore_then(ident().map_with_span(|name, span| (name, span)))
        .then(args_parser)
        .then(ret_parser)
        .recover_with(skip_then_retry_until([T!['}']]))
        .map_with_span(|((name, args), (ty, body)), span| {
            (
                GlobalStatement::FunctionDefinition(name, args, ty, body),
                span,
            )
        })
}

pub fn main_parser() -> impl Parser<Token, Spanned<GlobalStatement>, Error = Simple<Token>> {
    just(T!["main"])
        .ignore_then(
            just(T!["{"])
                .ignore_then(statement_parser().repeated())
                .then_ignore(just(T!["}"])),
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
        .recover_with(skip_then_retry_until([]))
        .separated_by(just(T![';']).or(just(T!['\n'])).or_not())
        .then_ignore(end())
}
