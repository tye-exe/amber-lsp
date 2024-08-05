use chumsky::prelude::*;
use text::{ident, keyword};

use super::{
    statements::statement_parser, FunctionArgument, GlobalStatement, ImportContent, TypeAnnotation,
};

pub fn import_parser() -> impl Parser<char, GlobalStatement, Error = Simple<char>> {
    let import_all_parser = just("*").padded().map(|_| ImportContent::ImportAll);

    let import_specific_parser = just("{")
        .ignore_then(ident().padded().separated_by(just(",")))
        .then_ignore(just("}"))
        .map(ImportContent::ImportSpecific);

    let path_parser = just('"')
        .ignore_then(filter(|c: &char| *c != '"').repeated().collect())
        .then_ignore(just('"'))
        .padded();

    just("import")
        .padded()
        .ignore_then(import_all_parser.or(import_specific_parser))
        .then(path_parser)
        .map(|(vars, path)| GlobalStatement::Import(vars, path))
}

pub fn function_parser() -> impl Parser<char, GlobalStatement, Error = Simple<char>> {
    let type_parser = just(':')
        .padded()
        .ignore_then(ident().padded())
        .padded()
        .map(TypeAnnotation::Type);

    let generic_arg_parser = ident().padded().map(FunctionArgument::Generic);
    let typed_arg_parser = ident()
        .padded()
        .then(type_parser.padded())
        .map(|(name, ty)| FunctionArgument::Typed(name, ty));

    let arg_parser = typed_arg_parser.or(generic_arg_parser);

    let args_parser = just("(")
        .padded()
        .ignore_then(arg_parser.separated_by(just(",")))
        .then_ignore(just(")").padded());

    let ret_parser = type_parser.or_not().padded().then(
        just("{")
            .padded()
            .ignore_then(statement_parser().padded().repeated())
            .then_ignore(just("}"))
            .padded(),
    );

    keyword("fun")
        .padded()
        .ignore_then(ident().padded())
        .then(args_parser)
        .then(ret_parser)
        .map(|((name, args), (ty, body))| GlobalStatement::FunctionDefinition(name, args, ty, body))
}

pub fn main_parser() -> impl Parser<char, GlobalStatement, Error = Simple<char>> {
    keyword("main")
        .padded()
        .then(
            just("{")
                .padded()
                .ignore_then(statement_parser().repeated())
                .then_ignore(just("}"))
                .padded(),
        )
        .map(|(_, body)| GlobalStatement::Main(body))
}

pub fn global_statement_parser() -> impl Parser<char, GlobalStatement, Error = Simple<char>> {
    let statement = statement_parser()
        .repeated()
        .map(GlobalStatement::Statement);

    import_parser()
        .or(function_parser())
        .or(main_parser())
        .or(statement)
}
