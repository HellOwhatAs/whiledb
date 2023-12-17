use crate::{interpreter::*, method};

/// add the type `NoneType` to buildin-state and return the state
pub fn buildin_none(state: Any) -> Result<Any> {
    let attrs = method!{
        __bool__(state, _self) {
            utils::get_buildin_var("false", state.clone())
        }
        __int__(state, _self) {
            Ok(obj_int::bigint2intinstance(BigInt::from(0), state))
        }
        __string__(state, _self) {
            Ok(obj_string::build_string("None", state))
        }
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state.clone(), 
        "NoneType", 
        res.clone()
    )?;
    utils::set_attr(
        state.clone(),
        "None", 
        Rc::new(RefCell::new(WdAny::Obj(Object{
            buildin: BuildIn::Bool(false),
            attrs: maplit::hashmap! {
                "__type__".to_string() => utils::get_buildin_var("NoneType", state.clone())?
            }
        })))
    )?;
    Ok(res)
}

pub fn buildin_none_post(type_obj: Any, state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?,
        "__name__".to_string() => obj_string::build_string("NoneType", state.clone())
    };
    for (k, v) in attrs.into_iter() {
        utils::set_attr(type_obj.clone(), &k, v)?;
    }
    Ok(())
}