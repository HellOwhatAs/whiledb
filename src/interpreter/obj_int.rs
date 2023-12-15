use num_bigint::BigInt;
use maplit;
use crate::interpreter::*;

/// add the type `int` to buildin-state and return the state
pub fn buildin_int(state: Any) -> Any {
    let attrs = maplit::hashmap! {
        "__add__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_add)))
        )),
        "__type__".to_string() => utils::state2typeobj("type", state.clone())
    };
    match &mut *state.borrow_mut() {
        WdAny::Obj(o) => {
            o.attrs.insert(
                "int".to_string(),
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

pub fn build_int(raw: &str, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::Int(raw.parse::<BigInt>().unwrap()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::state2typeobj("int", state.clone())
        }
    })))
}

fn int_add(mut args: VecDeque<Any>, state: Any) -> Result<Any, String> {
    let left = args.pop_back().unwrap();
    let right = args.pop_back().unwrap();
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
                    "__type__".to_string() => utils::state2typeobj("int", state.clone())
                }
            }))))
        },
        _ => unreachable!()
    }
}