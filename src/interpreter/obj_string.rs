use maplit;
use num::ToPrimitive;
use crate::{interpreter::*, method};


pub fn buildin_string(state: Any) -> Result<Any> {
    let attrs = method!{
        __init__(state, _self, arg) {
            match utils::get_father_attr(arg.clone(), "__string__") {
                Some(f) => utils::call(f, VecDeque::from([arg]), state),
                None => bail!("cannot convert arg to string"),
            }
        }
        __add__(state, s, other) {
            match (any2string(s), any2string(other)) {
                (Some(s1), Some(s2)) => Ok(build_string(&(s1 + &s2), state)),
                _ => bail!("cannot add string with other")
            }
        }
        __getitem__(state, _self, idx) {
            let s: VecDeque<char> = match any2string(_self.clone()) {
                Some(s) => s.chars().collect(),
                None => unreachable!(),
            };
            match &*idx.clone().borrow() {
                WdAny::Obj(o) => match &o.buildin {
                    BuildIn::Tuple(t) => {
                        match t.len() {
                            1 => {
                                let idx = match obj_int::wdany2bigint(&*t[0].borrow()) {
                                    Some(idx) => match idx.to_usize() {
                                        Some(idx) => idx,
                                        None => bail!("index overflow usize"),
                                    },
                                    None => bail!("index for string must be int"),
                                };
                                match s.get(idx) {
                                    Some(res) => Ok(build_string(&res.to_string(), state)),
                                    None => bail!("string index out of range"),
                                }
                            }
                            2 => {
                                let (idxs, idxe) = match (obj_int::wdany2bigint(&*t[0].borrow()), obj_int::wdany2bigint(&*t[1].borrow())) {
                                    (Some(idxs), Some(idxe)) => match (idxs.to_usize(), idxe.to_usize()) {
                                        (Some(idxs), Some(idxe)) => (idxs, idxe),
                                        _ => bail!("index overflow usize"),
                                    },
                                    _ => bail!("slice for string must be int"),
                                };
                                if idxs > idxe || idxe > s.len() {
                                    bail!("string index out of range")
                                }
                                let res: String = s.range(idxs..idxe).collect();
                                Ok(build_string(&res, state.clone()))
                            }
                            _ => bail!("invalid string index")
                        }
                    },
                    _ => bail!("index for getitem of string can only be int")
                },
                _ => bail!("index for getitem of string can only be int")
            }
        }
        len(state, s) {
            match any2string(s) {
                Some(s) => Ok(obj_int::bigint2intinstance(BigInt::from(s.chars().count()), state)),
                None => unreachable!(),
            }
        }
        __bool__(state, arg) {
            match any2string(arg) {
                Some(s) => match s.len() {
                    0 => utils::get_buildin_var("false", state),
                    _ => utils::get_buildin_var("true", state)
                },
                None => unreachable!(),
            }
        }
        __int__(state, arg) {
            match any2string(arg) {
                Some(s) => Ok(obj_int::bigint2intinstance(s.parse::<BigInt>()?, state)),
                None => unreachable!(),
            }
        }
        __string__(_state, arg) {
            Ok(arg)
        }
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state, 
        "string", 
        res.clone()
    )?;
    Ok(res)
}

pub fn buildin_string_post(type_obj: Any, state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?,
        "__name__".to_string() => obj_string::build_string("string", state.clone())
    };
    for (k, v) in attrs.into_iter() {
        utils::set_attr(type_obj.clone(), &k, v)?;
    }
    Ok(())
}


pub fn build_string_raw(raw: &str, state: Any) -> Result<Any> {
    Ok(Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::String(syn::parse_str::<syn::LitStr>(raw)?.value()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("string", state.clone()).unwrap()
        }
    }))))
}

pub fn build_string(s: &str, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::String(s.to_string()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("string", state.clone()).unwrap()
        }
    })))
}

pub fn any2string(x: Any) -> Option<String> {
    match &*x.clone().borrow() {
        WdAny::Obj(o) => match &o.buildin {
            BuildIn::String(s) => Some(s.clone()),
            _ => None
        },
        _ => None,
    }
}