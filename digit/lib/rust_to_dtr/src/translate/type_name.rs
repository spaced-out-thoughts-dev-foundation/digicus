extern crate syn;

use crate::common::compilation_state::CompilationState;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::translate::expression::parse_expression;
use crate::translate::rust_to_dtr_term::map_name;

pub fn figure_out_type(ty: &syn::Type) -> Result<String, NotTranslatableError> {
    let result_name: Result<String, NotTranslatableError> = match ty {
        syn::Type::Path(type_path) => Ok(format!("{}", parse_path(&type_path.path))),
        syn::Type::Never(_) => Ok(format!("!")),
        syn::Type::Ptr(ptr) => {
            if ptr.const_token.is_some() {
                match figure_out_type(&ptr.elem) {
                    Ok(val) => return Ok(format!("*const {}", val)),
                    Err(_) => {
                        return Err(NotTranslatableError::Custom(
                            "Could not figure out type for const pointer".to_string(),
                        ))
                    }
                }
            } else {
                match figure_out_type(&ptr.elem) {
                    Ok(val) => return Ok(format!("*mut {}", val)),
                    Err(_) => {
                        return Err(NotTranslatableError::Custom(
                            "Could not figure out type for non const pointer".to_string(),
                        ))
                    }
                }
            }
        }
        syn::Type::BareFn(bare_fn) => {
            let mut fn_str = String::new();
            fn_str.push_str("fn(");
            let mut args: Vec<String> = Vec::new();
            for arg in &bare_fn.inputs {
                match arg {
                    syn::BareFnArg {
                        attrs: _,
                        name: _,
                        ty: pat_type,
                    } => {
                        let val = figure_out_type(&pat_type);
                        match val {
                            Ok(val) => args.push(val),
                            Err(_) => {
                                return Err(NotTranslatableError::Custom(
                                    "Could not figure out type for BareFn".to_string(),
                                ));
                            }
                        }
                    }
                }
            }
            fn_str.push_str(&args.join(", "));
            fn_str.push_str(") -> ");
            if let syn::ReturnType::Type(_, ty) = &bare_fn.output {
                match figure_out_type(ty) {
                    Ok(val) => fn_str.push_str(val.as_str()),
                    Err(_) => {
                        return Err(NotTranslatableError::Custom(
                            "Could not figure out type for ReturnType".to_string(),
                        ))
                    }
                }
            }
            return Ok(fn_str);
        }
        syn::Type::TraitObject(trait_type) => {
            let mut trait_str = String::new();
            if trait_type.dyn_token.is_some() {
                trait_str.push_str("dyn ");
            }
            let mut bounds: Vec<String> = Vec::new();
            for bound in &trait_type.bounds {
                if let syn::TypeParamBound::Trait(trait_bound) = bound {
                    let path = &trait_bound.path;
                    let path_str = parse_path(path);
                    bounds.push(path_str);
                }
            }
            trait_str.push_str(&bounds.join(" + "));
            return Ok(trait_str);
        }
        syn::Type::Reference(ref_type) => {
            let type_string = figure_out_type(&ref_type.elem);

            return Ok(format!("&{}", map_name(&type_string.unwrap())?));
        }
        syn::Type::Tuple(tuple_type) => {
            let mut tuple_str = String::new();
            tuple_str.push_str("(");
            let mut tuple_types: Vec<String> = Vec::new();
            for elem in &tuple_type.elems {
                let val = figure_out_type(elem);
                match val {
                    Ok(val) => tuple_types.push(map_name(&val)?),
                    Err(_) => {
                        return Err(NotTranslatableError::Custom(
                            "Could not figure out type for tuple".to_string(),
                        ))
                    }
                }
            }
            tuple_str.push_str(&tuple_types.join(", "));
            tuple_str.push_str(")");
            return Ok(tuple_str);
        }
        _ => Ok(format!("idk")),
    };

    match result_name {
        Ok(val) => map_name(&val),
        Err(_) => Err(NotTranslatableError::Custom(
            "Could not figure out type misc?".to_string(),
        )),
    }
}

fn parse_angle_bracketed_generic_arguments(args: &syn::AngleBracketedGenericArguments) -> String {
    let mut args_list: Vec<String> = Vec::new();

    for arg in &args.args {
        match arg {
            syn::GenericArgument::Type(ty) => {
                let val = figure_out_type(ty);

                match val {
                    Ok(val) => args_list.push(map_name(&val).unwrap()),
                    Err(_) => {
                        return "Could not figure out type for angle bracketed type".to_string();
                    }
                }
            }
            syn::GenericArgument::Const(constant) => {
                // TODO: fix this hackery and put all of it in one place
                let value = parse_expression(constant, &mut CompilationState::new()).unwrap()[0]
                    .input[0]
                    .clone();
                args_list.push(format!("{}", map_name(&value).unwrap()));
            }
            _ => {}
        }
    }

    args_list.join(", ")
}

pub fn parse_path(path: &syn::Path) -> String {
    let segments = &path.segments;
    let segment = &segments[0];

    let path_str = parse_path_segment(segment);
    if segments.len() > 1 {
        return format_segment_name(
            segments
                .into_iter()
                .map(|segment| format!("{}", segment.ident.to_string()))
                .collect::<Vec<String>>()
                .join("::"),
        );
    }

    format!("{}", format_segment_name(path_str))
}

fn parse_path_segment(segment: &syn::PathSegment) -> String {
    match &segment.arguments {
        syn::PathArguments::None => {
            return format!("{}", segment.ident);
        }
        syn::PathArguments::AngleBracketed(args) => {
            let mut path_str = String::new();
            path_str.push_str(&format!("{}", segment.ident.to_string()));
            path_str.push_str("<");
            path_str.push_str(&parse_angle_bracketed_generic_arguments(args));
            path_str.push_str(">");

            return path_str;
        }
        _ => {}
    }

    format!("{}", segment.ident)
}

fn format_segment_name(mut segment_name: String) -> String {
    segment_name = segment_name.replace("Self::", "");

    map_name(&segment_name).unwrap_or(segment_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figure_out_type_primitive_i32() {
        let ty = syn::parse_str("i32").unwrap();
        assert_eq!(super::figure_out_type(&ty), Ok("Integer".to_string()));
    }

    #[test]
    fn test_figure_out_type_primitive_bool() {
        let ty = syn::parse_str("bool").unwrap();
        assert_eq!(super::figure_out_type(&ty), Ok("Boolean".to_string()));
    }

    #[test]
    fn test_figure_out_type_primitive_char() {
        let ty = syn::parse_str("char").unwrap();
        assert_eq!(super::figure_out_type(&ty), Ok("String".to_string()));
    }

    #[test]
    fn test_figure_out_type_primitive_f32() {
        let ty = syn::parse_str("f32").unwrap();
        assert_eq!(super::figure_out_type(&ty), Ok("Float".to_string()));
    }

    #[test]
    fn test_figure_out_type_primitive_f64() {
        let ty = syn::parse_str("f64").unwrap();
        assert_eq!(super::figure_out_type(&ty), Ok("Float".to_string()));
    }

    mod collections {
        use super::*;

        #[test]
        fn test_figure_out_type_vec_i32() {
            let ty = syn::parse_str("Vec<Integer>").unwrap();
            assert_eq!(super::figure_out_type(&ty), Ok("List<Integer>".to_string()));
        }

        #[test]
        fn test_figure_out_type_hash_map() {
            let ty = syn::parse_str("HashMap<i32, i32>").unwrap();
            assert_eq!(
                super::figure_out_type(&ty),
                Ok("Dictionary<Integer, Integer>".to_string())
            );
        }
    }

    #[test]
    fn test_figure_out_type_enum() {
        let ty = syn::parse_str("Option<Integer>").unwrap();
        assert_eq!(
            super::figure_out_type(&ty),
            Ok("Option<Integer>".to_string())
        );
    }

    #[test]
    fn test_figure_out_type_pointer() {
        let ty = syn::parse_str("*const i32").unwrap();
        assert_eq!(
            super::figure_out_type(&ty),
            Ok("*const Integer".to_string())
        );
    }

    #[test]
    fn test_figure_out_type_function() {
        let ty = syn::parse_str("fn(i32) -> i32").unwrap();
        assert_eq!(
            super::figure_out_type(&ty),
            Ok("fn(Integer) -> Integer".to_string())
        );
    }

    #[test]
    fn test_figure_out_type_never() {
        let ty = syn::parse_str("!").unwrap();
        assert_eq!(
            super::figure_out_type(&ty),
            Err(NotTranslatableError::Custom(
                "Unable to translate !".to_string()
            ))
        );
    }
}
