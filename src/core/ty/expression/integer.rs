use crate::core::{
    ast::{parse_ast, ty_ast::*},
    ty::{Ty, TyError},
};

impl parse_ast::Integer {
    pub fn ty_solve(self) -> Result<Integer, TyError> {
        Ok(Integer {
            value: self.value,
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Int,
                return_ty: None,
            },
        })
    }
}

#[cfg(test)]
mod test_integer {
    use crate::{
        core::{ast::parse_ast::*, ty::Ty},
        util::source::Span,
    };

    #[test]
    fn integer_infer() {
        assert_eq!(
            Integer::new(0, Span::default())
                .ty_solve()
                .unwrap()
                .ty_info
                .ty,
            Ty::Int
        );
    }

    #[test]
    fn integer_return() {
        assert_eq!(
            Integer::new(0, Span::default())
                .ty_solve()
                .unwrap()
                .ty_info
                .return_ty,
            None,
        );
    }
}
