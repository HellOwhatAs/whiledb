use crate::interpreter::*;

/// add the type `type` to buildin-state and return the state
pub fn buildin_type(state: Any) -> Any {
    let attrs = maplit::hashmap! {
        "__name__".to_string() => Rc::new(RefCell::new(WdAny::Obj(Object{
            buildin: BuildIn::String("type".to_string()),
            attrs: HashMap::new()
        }))),
        // "__init__".to_string() => Rc::new(RefCell::new(
        //     WdAny::Func(Function::BuildInFunction(BuildInFunction(int_add)))
        // )),
    };
    match &mut *state.borrow_mut() {
        WdAny::Obj(o) => {
            o.attrs.insert(
                "type".to_string(),
                Rc::new(RefCell::new(WdAny::Obj(Object{
                    buildin: BuildIn::Not,
                    attrs: attrs
                })))
            );
        },
        _ => unreachable!(),
    }
    state
}