pub mod ast;
pub mod config;
pub mod token;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Statement, r#type::{Type, Primitive}, func::Function, stream::Stream},
        token::tokenise,
    };

    #[test]
    fn empty_static_function() -> Result<(), eyre::Report> {
        let input = "static function test() {}";
        let mut ast = vec![];
        let mut result = Stream::new(tokenise(&mut (&mut input.chars()).into(), None)?).parse(&mut ast)?;
        ast.append(&mut result);

        let expected = vec![Statement::Function(Function {
            function_name: "test".to_string(),
            arguments: vec![],
            return_type: Type::Primitive(Primitive::Void),
            contents: vec![],
            is_static: true,
        })];

        assert_eq!(ast, expected);

        Ok(())
    }
}
