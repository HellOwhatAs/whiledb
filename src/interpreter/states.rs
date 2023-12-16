use crate::interpreter::*;


pub fn init_state() -> Result<Any> {
    let initial_attrs = maplit::hashmap! {
        "None".to_string() => Rc::new(RefCell::new(WdAny::Obj(Object{
            buildin: BuildIn::Not,
            attrs: HashMap::new()
        }))),
    };
    let state = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: initial_attrs
    })));
    let type_type_obj = obj_type::buildin_type(state.clone())?;
    let type_bool_obj = obj_bool::buildin_bool(state.clone())?;
    let type_int_obj = obj_int::buildin_int(state.clone())?;
    let type_string_obj = obj_string::buildin_string(state.clone())?;
    utils::set_attr(type_type_obj, "__name__", obj_string::build_string("type", state.clone()))?;
    utils::set_attr(type_bool_obj, "__name__", obj_string::build_string("bool", state.clone()))?;
    utils::set_attr(type_int_obj, "__name__", obj_string::build_string("int", state.clone()))?;
    utils::set_attr(type_string_obj, "__name__", obj_string::build_string("string", state.clone()))?;
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
    for arg in args.iter() {
        match &*obj_string::string_init(VecDeque::from([utils::get_buildin_var("None", state.clone())?, arg.clone()]), state.clone())?.borrow() {
            WdAny::Obj(o) => match &o.buildin {
                BuildIn::String(s) => print!("{} ", s),
                _ => unreachable!()
            },
            _ => unreachable!(),
        }
    }
    println!();
    utils::get_buildin_var("None", state)
}