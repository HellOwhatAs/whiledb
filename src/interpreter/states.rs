use crate::interpreter::*;


pub fn init_state() -> Result<Any> {
    let initial_attrs = maplit::hashmap! {
    };
    let state = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: initial_attrs
    })));
    let type_none_obj = obj_none::buildin_none(state.clone())?;
    let type_type_obj = obj_type::buildin_type(state.clone())?;
    let type_bool_obj = obj_bool::buildin_bool(state.clone())?;
    let type_int_obj = obj_int::buildin_int(state.clone())?;
    let type_float_obj = obj_float::buildin_float(state.clone())?;
    let type_string_obj = obj_string::buildin_string(state.clone())?;
    let type_list_obj = obj_list::buildin_list(state.clone())?;
    obj_none::buildin_none_post(type_none_obj, state.clone())?;
    obj_type::buildin_type_post(type_type_obj, state.clone())?;
    obj_bool::buildin_bool_post(type_bool_obj, state.clone())?;
    obj_int::buildin_int_post(type_int_obj, state.clone())?;
    obj_float::buildin_float_post(type_float_obj, state.clone())?;
    obj_string::buildin_string_post(type_string_obj, state.clone())?;
    obj_list::buildin_list_post(type_list_obj, state.clone())?;
    let dependent_attrs = maplit::hashmap! {
        "print".to_string() => Rc::new(RefCell::new(
            WdAny::Func("print".to_string(), Function::BuildInFunction(BuildInFunction(buildin_print)))
        )),
    };
    for (k, v) in dependent_attrs.into_iter() {
        utils::set_attr(state.clone(), &k, v)?;
    }
    Ok(state)
}


fn buildin_print(args: VecDeque<Any>, state: Any) -> Result<Any> {
    for arg in args.into_iter() {
        let tmp = utils::convert2string(arg, state.clone())?;
        print!("{} ", tmp);
    }
    println!();
    utils::get_buildin_var("None", state)
}