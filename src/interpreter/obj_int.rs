use num::BigInt;
use maplit;
use crate::interpreter::*;

/// add the type `int` to buildin-state and return the state
pub fn buildin_int(state: Any) -> Result<(), String> {
    let attrs = maplit::hashmap! {
        "__add__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_add)))
        )),
        "__sub__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_sub)))
        )),
        "__mul__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_mul)))
        )),
        "__div__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_div)))
        )),
        "__mod__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_mod)))
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

fn wdany2bigint(x: &WdAny) -> Option<&BigInt> {
    match x {
        WdAny::Obj(o) => {
            match &o.buildin {
                BuildIn::Int(i) => Some(i),
                _ => None
            }
        },
        _ => None
    }
}

fn bigint2intinstance(x: BigInt, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Int(x),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("int", state.clone()).unwrap()
        }
    })))
}

fn int_add(args: &VecDeque<Any>, state: Any) -> Result<Any, String> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 + i2, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__radd__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => Err(format!("Cannot add {:?} and {:?}", left, right)),
        },
        _ => unreachable!(),
    }
}

fn int_sub(args: &VecDeque<Any>, state: Any) -> Result<Any, String> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 - i2, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rsub__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => Err(format!("Cannot sub {:?} and {:?}", left, right)),
        },
        _ => unreachable!(),
    }
}

fn int_mul(args: &VecDeque<Any>, state: Any) -> Result<Any, String> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 * i2, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rmul__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => Err(format!("Cannot mul {:?} and {:?}", left, right)),
        },
        _ => unreachable!(),
    }
}

fn int_div(args: &VecDeque<Any>, state: Any) -> Result<Any, String> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(
            if num::Zero::is_zero(i2) {
                return Err("Cannot div zero".to_string())
            } else { i1 / i2 }, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rdiv__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => Err(format!("Cannot div {:?} and {:?}", left, right)),
        },
        _ => unreachable!(),
    }
}

fn int_mod(args: &VecDeque<Any>, state: Any) -> Result<Any, String> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(
            if num::Zero::is_zero(i2) {
                return Err("Cannot mod zero".to_string())
            } else { i1 % i2 }, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rmod__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => Err(format!("Cannot mod {:?} and {:?}", left, right)),
        },
        _ => unreachable!(),
    }
}
