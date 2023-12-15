use crate::interpreter::*;

/// add the type `type` to buildin-state and return the state
pub fn buildin_type(state: Any) -> Result<(), String> {
    let attrs = maplit::hashmap! {
        "__name__".to_string() => Rc::new(RefCell::new(WdAny::Obj(Object{
            buildin: BuildIn::String("type".to_string()),
            attrs: HashMap::new()
        })))
    };
    utils::set_attr(
        state, 
        "type", 
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Not,
                attrs: attrs
            }
    ))))
}