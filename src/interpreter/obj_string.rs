use maplit;
use crate::interpreter::*;


pub fn buildin_string(state: Any) -> Result<Any> {
    let attrs = maplit::hashmap! {
        "__init__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__init__".to_string(), Function::BuildInFunction(BuildInFunction(string_init)))
        )),
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state, 
        "string", 
        res.clone()
    )?;
    Ok(res)
}


pub fn build_string_raw(raw: &str, state: Any) -> Result<Any> {
    Ok(Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::String(syn::parse_str::<syn::LitStr>(raw)?.value()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("string", state.clone()).unwrap()
        }
    }))))
}

pub fn build_string(s: &str, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::String(s.to_string()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("string", state.clone()).unwrap()
        }
    })))
}

pub fn string_init(args: VecDeque<Any>, state: Any) -> Result<Any> {
    match args.len() {
        1 => Ok(build_string("", state)),
        2 => {
            let arg = args[1].clone();
            match &*arg.clone().borrow() {
                WdAny::Obj(o) => {
                    match &o.buildin {
                        BuildIn::Bool(b) => Ok(build_string(&format!("{}", b), state)),
                        BuildIn::Int(i) => Ok(build_string(&i.to_string(), state)),
                        BuildIn::String(_) => Ok(arg),
                        _ => match utils::get_attr(arg.clone(), "__type__") {
                            Some(t) => match Rc::ptr_eq(&t, &utils::get_buildin_var("type", state.clone())?) {
                                true => match utils::get_attr(arg.clone(), "__name__") {
                                    Some(name) => Ok(name),
                                    None => unreachable!("an type instance without a name ? OMG"),
                                },
                                false => match utils::get_attr(arg.clone(), "__string__") {
                                    Some(f) => {
                                        let mut args = args;
                                        args.pop_front();
                                        utils::call(f, args, state)
                                    },
                                    None => bail!("cannot convert arg to string"),
                                },
                            },
                            None => match Rc::ptr_eq(&arg, &utils::get_buildin_var("type", state.clone())?) {
                                true => Ok(build_string("type", state)),
                                false => match Rc::ptr_eq(&arg, &utils::get_buildin_var("None", state.clone())?) {
                                    true => Ok(build_string("None", state)),
                                    false => unreachable!("an object without a type ? OMG"),
                                },
                            },
                        }
                    }
                },
                WdAny::Func(fname, f) => {
                    match f {
                        Function::BuildInFunction(_) => Ok(build_string(&format!("<buildin-func {fname}>"), state)),
                        Function::DefinedFunction(_) => Ok(build_string(&format!("<func {fname}>"), state)),
                    }
                },
            }
        }
        _ => bail!("__init__ of string accepts at most 2 argument")
    }
}