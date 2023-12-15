use num_bigint::BigInt;
use maplit;
use crate::interpreter::*;

/// add the type `int` to buildin-state and return the state
pub fn buildin_int(state: Any) -> Result<(), String> {
    let attrs = maplit::hashmap! {
        "__add__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_add)))
        )),
        "__type__".to_string() => utils::get_buildin_var("type", state.clone()).unwrap()
    };
    utils::set_attr(
        state, 
        "int", 
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Not,
                attrs: attrs
            }
    ))))
}

pub fn build_int(raw: &str, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::Int(raw.parse::<BigInt>().unwrap()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("int", state.clone()).unwrap()
        }
    })))
}

fn int_add(args: &VecDeque<Any>, state: Any) -> Result<Any, String> {
    let (left, right) = (args[0].clone(), args[1].clone());
    match (&*left.clone().borrow(), &*right.clone().borrow()) {
        (WdAny::Obj(o1), WdAny::Obj(o2)) => {
            let res = match (&o1.buildin, &o2.buildin) {
                (BuildIn::Int(i1), BuildIn::Int(i2)) =>
                    BigInt::checked_add(i1, i2).unwrap(),
                _ => unreachable!()
            };
            Ok(Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Int(res),
                attrs: maplit::hashmap! {
                    "__type__".to_string() => utils::get_buildin_var("int", state.clone()).unwrap()
                }
            }))))
        },
        _ => unreachable!()
    }
}