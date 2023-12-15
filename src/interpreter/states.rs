use crate::interpreter::*;


pub fn init_state() -> Any {
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
    obj_type::buildin_type(state.clone()).unwrap();
    obj_bool::buildin_bool(state.clone()).unwrap();
    obj_int::buildin_int(state.clone()).unwrap();
    state
}


fn buildin_print(args: &VecDeque<Any>, state: Any) -> Result<Any, String> {
    for arg in args.iter() {
        print!("{:?} ", arg.clone());
    }
    println!();
    Ok(utils::get_buildin_var("None", state)?)
}