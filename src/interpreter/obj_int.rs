use num_bigint::BigInt;
use maplit;
use crate::interpreter::*;
use crate::ast::*;

pub fn build_int_type() -> Any {
    let attrs = maplit::hashmap! {
        "__add__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_add)))
        )),
    };
    todo!()
}

pub fn state2typeobj(state: Any) -> Any {
    todo!()
}

pub fn build_int(raw: &str, state: Any) -> (Any, Any) {
    (
        Rc::new(RefCell::new(WdAny::Obj(Object {
            buildin: BuildIn::Int(raw.parse::<BigInt>().unwrap()),
            attrs: maplit::hashmap! {
                "__type__".to_string() => state2typeobj(state.clone())
            }
        }))),
        state
    )
}

pub fn int_add(mut args: VecDeque<Any>, state: Any) -> (Any, Any) {
    let left = args.pop_back().unwrap();
    let right = args.pop_back().unwrap();
    match (&mut *left.clone().borrow_mut(), &mut *right.clone().borrow_mut()) {
        (WdAny::Obj(o1), WdAny::Obj(o2)) => {
            let res = match (&mut o1.buildin, &mut o2.buildin) {
                (BuildIn::Int(i1), BuildIn::Int(i2)) =>
                    BigInt::checked_add(i1, i2).unwrap(),
                _ => unreachable!()
            };
            (
                Rc::new(RefCell::new(WdAny::Obj(Object{
                    buildin: BuildIn::Int(res),
                    attrs: maplit::hashmap! {
                        "__type__".to_string() => state2typeobj(state.clone())
                    }
                }))),
                state
            )
        },
        _ => unreachable!()
    }
}