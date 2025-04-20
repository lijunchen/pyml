use crate::tast;
use ::ast::ast;

pub fn mangle_impl_name(trait_name: &ast::Uident, for_ty: &tast::Ty, method_name: &str) -> String {
    // Create a unique string representation of the type `for_ty`. Handle complex types carefully.
    let for_ty_str = match for_ty {
        tast::Ty::TUnit => "Unit".to_string(),
        tast::Ty::TBool => "Bool".to_string(),
        tast::Ty::TInt => "Int".to_string(),
        tast::Ty::TString => "String".to_string(),
        tast::Ty::TTuple { typs } => {
            let inner = typs
                .iter()
                .map(|ty| format!("{:?}", ty))
                .collect::<Vec<_>>()
                .join("_");
            format!("Tuple_{}", inner)
        }
        tast::Ty::TApp { name, args } => {
            let inner = args
                .iter()
                .map(|ty| format!("{:?}", ty))
                .collect::<Vec<_>>()
                .join("_");
            format!("App_{}_{}", name.0, inner)
        }
        tast::Ty::TParam { .. } => {
            unreachable!()
        }
        tast::Ty::TFunc { .. } => {
            unreachable!()
        }
        tast::Ty::TVar(..) => {
            unreachable!()
        }
    };
    format!("impl_{}_{}_{}", trait_name.0, for_ty_str, method_name)
}
