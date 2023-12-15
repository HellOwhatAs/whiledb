use crate::interpreter::*;


pub fn init_state() -> Any {
    let initial_attrs = maplit::hashmap! {
        
    };
    let state = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: initial_attrs
    })));
    obj_type::buildin_type(state.clone());
    obj_int::buildin_int(state.clone());
    state
}