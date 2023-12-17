use crate::{interpreter::*, method};

/// add the type `type` to buildin-state and return the state
pub fn buildin_type(state: Any) -> Result<Any> {
    let attrs = method!{
        __init__(state, _self, arg) {
            match Rc::ptr_eq(&arg, &utils::get_buildin_var("type", state.clone())?) {
                true => Ok(arg),
                false => match utils::get_attr(arg.clone(), "__type__") {
                    Some(t) => Ok(t),
                    None => bail!("a object without a type")
                },
            }
        }
        __string__(_state, _self) {
            match utils::get_attr(_self, "__name__") {
                Some(name) => Ok(name),
                None => unreachable!(),
            }
        }
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

pub fn buildin_type_post(type_obj: Any, state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__name__".to_string() => obj_string::build_string("type", state.clone()),
        "__type__".to_string() => Rc::new(RefCell::new(WdAny::Obj(Object {
            buildin: BuildIn::Not,
            attrs: method!{
                __string__(state, _self) {
                    Ok(obj_string::build_string("type", state))
                }
            }
        })))
    };
    for (k, v) in attrs.into_iter() {
        utils::set_attr(type_obj.clone(), &k, v)?;
    }
    Ok(())
}