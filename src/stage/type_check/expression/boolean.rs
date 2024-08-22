use super::*;

impl parse_ast::Boolean {
    pub fn ty_solve(self) -> Result<Boolean, TyError> {
        Ok(Boolean {
            value: self.value,
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Boolean,
                return_ty: None,
            },
        })
    }
}

#[cfg(test)]
mod test_boolean {
    use crate::{
        repr::{ast::untyped::*, ty::Ty},
        util::span::Span,
    };

    #[test]
    fn boolean_infer() {
        assert_eq!(
            Boolean::new(false, Span::default(), Default::default())
                .ty_solve()
                .unwrap()
                .ty_info
                .ty,
            Ty::Boolean
        );
    }

    #[test]
    fn boolean_return() {
        assert_eq!(
            Boolean::new(false, Span::default(), Default::default())
                .ty_solve()
                .unwrap()
                .ty_info
                .return_ty,
            None,
        );
    }
}
