use crate::{codegen::*, core::ast::Expression};

mod literal;
mod name;
mod operator;
mod tuple;

impl<'a> Walker<Expression> for Context<'a> {
    fn walk(&mut self, expression: Expression) -> Result<(), Error> {
        match expression {
            Expression::Match => {
                todo!("match is not implemented now")
            }
            Expression::Closure => {
                todo!("closure is not implemented now")
            }
            Expression::Literal(literal) => self.walk(literal),
            Expression::Path => {
                todo!("path is not implemented now")
            }
            Expression::Array => {
                todo!("array is not implemented now")
            }
            Expression::Tuple(expressions) => self.walk(expressions),
            Expression::Init => {
                todo!("struct/union init is not implemented now")
            }
            Expression::Operator(operator) => self.walk(operator),
            Expression::Name(name) => self.walk(name),
            Expression::If(r#if) => todo!("if is not implemented now"),
        }
    }
}
