use crate::util::scope::Scope;

use super::*;

impl parse_ast::Loop {
    pub fn ty_solve(self, compiler: &mut Compiler, scope: &mut Scope) -> Result<Loop, TyError> {
        // Type check the body
        let body = self.body.ty_solve(compiler, scope)?;

        // TODO: Temporary whilst can't break expression
        match body.ty_info.ty {
            Ty::Unit | Ty::Never => (),
            ty => {
                return Err(TyError::Mismatch(Ty::Unit, ty));
            }
        };

        Ok(Loop {
            ty_info: body.ty_info.clone(),
            body,
            span: self.span,
        })
    }
}
