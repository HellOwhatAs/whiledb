use crate::interpreter::*;


pub fn init_state() -> Result<Any> {
    let initial_attrs = maplit::hashmap! {
        "None".to_string() => Rc::new(RefCell::new(WdAny::Obj(Object{
            buildin: BuildIn::Not,
            attrs: HashMap::new()
        }))),
        "print".to_string() => Rc::new(RefCell::new(WdAny::Func(Function::BuildInFunction(BuildInFunction(buildin_print))))),
    };
    let state = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: initial_attrs
    })));
    obj_type::buildin_type(state.clone())?;
    obj_bool::buildin_bool(state.clone())?;
    obj_int::buildin_int(state.clone())?;
    Ok(state)
}


fn buildin_print(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    for arg in args.iter() {
        print!("{:?} ", arg.clone());
    }
    println!();
    utils::get_buildin_var("None", state)
}