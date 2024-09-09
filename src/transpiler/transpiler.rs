use crate::{
    lexer::Lexer,
    parser::{Expr, ParseError, Parser},
};

pub struct Transpiler {
    ast: Expr,
}

impl Transpiler {
    pub fn from_source<T: ToString>(content: T) -> Result<Self, ParseError> {
        let tokens = Lexer::new(content).tokenize();
        let ast = Parser::new(tokens).create_program()?;
        Ok(Self { ast })
    }
    fn transform_accessors(p: &Box<Expr>, child: &Box<Expr>) -> String {
        let mut buffer = String::new();
        let mut p = *p.clone();
        while let Expr::Accessor { parent, child } = p {
            match *child {
                Expr::Identifier(str) => {
                    buffer.insert_str(0, &format!("{}.", str));
                }
                _ => panic!("didnt expect this"),
            };
            p = *parent;
        }
        match *child.clone() {
            Expr::Identifier(s) => buffer.push_str(&s),
            _ => panic!("Expected child to be identifier"),
        };
        match p {
            Expr::Identifier(s) => buffer.insert_str(0, &format!("{}.", s)),
            _ => panic!("Expected the first parent to be a identifier"),
        }
        buffer
    }
    fn transform_fncall(fname: &Box<Expr>, parameters: &Box<Vec<Expr>>) -> String {
        let mut params = String::new();
        let fname = Self::transpile_expr(fname);
        for p in parameters.iter() {
            params.push_str(&format!("{},", Self::transpile_expr(p)))
        }
        params.pop();
        format!("{fname}({params});")
    }
    fn transpile_expr(expr: &Expr) -> String {
        match expr {
            Expr::Accessor { parent, child } => Self::transform_accessors(parent, child),
            Expr::FnCall { fname, params } => Self::transform_fncall(fname, params),
            Expr::StrLit(s) => format!("\"{}\"", s),
            Expr::Identifier(s) => s.to_string(),
            Expr::Globals(_) => panic!(
                "Globals cannot be transpiled because it makes no sense so i dont want it to"
            ),
        }
    }
    pub fn transpile(&self) -> String {
        match &self.ast {
            Expr::Globals(exprs) => {
                let mut result = String::new();

                for expr in exprs.iter() {
                    result.push_str(&Self::transpile_expr(expr))
                }
                result
            }
            expr => panic!("Unexpected initial expression {:?}", expr),
        }
    }
}
