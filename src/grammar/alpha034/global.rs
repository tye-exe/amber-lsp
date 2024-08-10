use chumsky::prelude::*;
use text::{ident, keyword};

use super::{
    statements::statement_parser, FunctionArgument, GlobalStatement, ImportContent, Span, Spanned,
    TypeAnnotation,
};

pub fn import_parser() -> impl Parser<char, Spanned<GlobalStatement>, Error = Simple<char>> {
    let import_all_parser = just("*").map_with_span(|_, span| (ImportContent::ImportAll, span));

    let import_specific_parser = just("{")
        .ignore_then(
            ident()
                .map_with_span(|name, span| (name, span))
                .padded()
                .separated_by(just(",")),
        )
        .then_ignore(just("}"))
        .map_with_span(|vars, span| (ImportContent::ImportSpecific(vars), span));

    let path_parser = just('"')
        .ignore_then(
            filter(|c: &char| *c != '"')
                .repeated()
                .map_with_span(|name, span| (name.iter().collect(), span)),
        )
        .then_ignore(just('"'));

    just("import")
        .ignore_then(import_all_parser.or(import_specific_parser).padded())
        .then(path_parser)
        .map_with_span(|(vars, path), span| (GlobalStatement::Import(vars, path), span))
}

pub fn function_parser() -> impl Parser<char, Spanned<GlobalStatement>, Error = Simple<char>> {
    let type_parser = just(':')
        .ignore_then(filter(|c: &char| c.is_whitespace()).repeated())
        .ignore_then(ident().map_with_span(|name, span| (name, span)))
        .map_with_span(|name, span| (TypeAnnotation::Type(name), span));

    let generic_arg_parser = ident()
        .map_with_span(|name, span: Span| (FunctionArgument::Generic((name, span.clone())), span));

    let typed_arg_parser = ident()
        .map_with_span(|name, span| (name, span))
        .then_ignore(filter(|c: &char| c.is_whitespace()).repeated())
        .then(type_parser)
        .map_with_span(|(name, ty), span| (FunctionArgument::Typed(name, ty), span));

    let arg_parser = typed_arg_parser.or(generic_arg_parser);

    let args_parser = arg_parser
        .padded()
        .separated_by(just(",").recover_with(skip_then_retry_until([')'])))
        .delimited_by(
            just('('),
            just(')').recover_with(skip_then_retry_until(['{', '}', ']'])),
        );

    let ret_parser = type_parser.or_not().then(
        just("{")
            .padded()
            .ignore_then(statement_parser().padded().repeated())
            .then_ignore(just("}").recover_with(skip_then_retry_until([]))),
    );

    keyword("fun")
        .ignore_then(ident().map_with_span(|name, span| (name, span)).padded())
        .then(args_parser.padded())
        .then(ret_parser)
        .recover_with(skip_then_retry_until(['}']))
        .map_with_span(|((name, args), (ty, body)), span| {
            (
                GlobalStatement::FunctionDefinition(name, args, ty, body),
                span,
            )
        })
}

pub fn main_parser() -> impl Parser<char, Spanned<GlobalStatement>, Error = Simple<char>> {
    keyword("main")
        .ignore_then(
            just("{")
                .padded()
                .ignore_then(statement_parser().padded().repeated())
                .then_ignore(just("}")),
        )
        .map_with_span(|body, span| (GlobalStatement::Main(body), span))
}

pub fn global_statement_parser(
) -> impl Parser<char, Vec<Spanned<GlobalStatement>>, Error = Simple<char>> {
    let statement = statement_parser()
        .padded()
        .map(|stmnt| (GlobalStatement::Statement(stmnt.clone()), stmnt.1));

    import_parser()
        .or(function_parser())
        .or(main_parser())
        .or(statement)
        .padded()
        .recover_with(skip_then_retry_until([]))
        .separated_by(just(';').or(just('\n')).padded().or_not())
        .then_ignore(end())
}
