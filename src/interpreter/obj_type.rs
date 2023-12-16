use crate::interpreter::*;

/// add the type `type` to buildin-state and return the state
pub fn buildin_type(state: Any) -> Result<Any> {
    let attrs = maplit::hashmap! {
        "__init__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__init__".to_string(), Function::BuildInFunction(BuildInFunction(type_init)))
        )),
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state, 
        "type", 
        res.clone()
    )?;
    Ok(res)
}

fn type_init(args: VecDeque<Any>, state: Any) -> Result<Any> {
    match args.len() {
        1 => utils::get_buildin_var("type", state),
        2 => {
            let arg = args[1].clone();
            match utils::get_attr(arg.clone(), "__type__") {
                Some(t) => Ok(t),
                None => match Rc::ptr_eq(&arg, &utils::get_buildin_var("type", state.clone())?) {
                    true => Ok(arg),
                    false => match Rc::ptr_eq(&arg, &utils::get_buildin_var("None", state)?) {
                        true => Ok(arg),
                        false => unreachable!("an object without a type ? OMG"),
                    },
                },
            }
        }
        _ => bail!("__init__ of type accepts at most 2 argument")
    }
}