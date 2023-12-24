use maplit;
use num::ToPrimitive;
use crate::{interpreter::*, method};

pub fn buildin_list(state: Any) -> Result<Any> {
    let attrs = method!{
        __init__(state, _self, arg) {
            match &*arg.clone().borrow() {
                WdAny::Obj(o) => match &o.buildin {
                    BuildIn::String(s) => Ok(
                        build_list(&VecDeque::from_iter(s.chars().map(|e| {
                            obj_string::build_string(&e.to_string(), state.clone())
                        })), state.clone())
                    ),
                    BuildIn::Tuple(_) => Ok(arg.clone()),
                    _ => bail!("cannot convert to list")
                },
                _ => bail!("cannot convert to list")
            }
        }
        len(state, _self) {
            match &*_self.clone().borrow() {
                WdAny::Obj(o) => match &o.buildin {
                    BuildIn::Tuple(t) => Ok(obj_int::bigint2intinstance(BigInt::from(t.len()), state.clone())),
                    _ => unreachable!()
                },
                _ => unreachable!(),
            }
        }
        append(state, _self, elem) {
            match &mut *_self.clone().borrow_mut() {
                WdAny::Obj(o) => match &mut o.buildin {
                    BuildIn::Tuple(t) => t.push_back(elem),
                    _ => unreachable!()
                },
                _ => unreachable!(),
            };
            utils::get_buildin_var("None", state)
        }
        pop(_state, _self) {
            match &mut *_self.clone().borrow_mut() {
                WdAny::Obj(o) => match &mut o.buildin {
                    BuildIn::Tuple(t) => match t.pop_back() {
                        Some(x) => Ok(x),
                        None => bail!("pop from empty list"),
                    },
                    _ => unreachable!()
                },
                _ => unreachable!(),
            }
        }
        __getitem__(state, _self, idx) {
            let vecdq = match any2vecdeque(_self.clone()) {
                Some(vecdq) => vecdq,
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
                                    None => bail!("index for list must be int"),
                                };
                                match vecdq.get(idx) {
                                    Some(res) => Ok(res.clone()),
                                    None => bail!("list index out of range"),
                                }
                            }
                            2 => {
                                let (idxs, idxe) = match (obj_int::wdany2bigint(&*t[0].borrow()), obj_int::wdany2bigint(&*t[1].borrow())) {
                                    (Some(idxs), Some(idxe)) => match (idxs.to_usize(), idxe.to_usize()) {
                                        (Some(idxs), Some(idxe)) => (idxs, idxe),
                                        _ => bail!("index overflow usize"),
                                    },
                                    _ => bail!("slice for list must be int"),
                                };
                                if idxs > idxe || idxe > vecdq.len() {
                                    bail!("list index out of range")
                                }
                                let res: VecDeque<_> = vecdq.range(idxs..idxe).map(|e| Rc::new(RefCell::new(e.borrow().clone()))).collect();
                                Ok(build_list(&res, state.clone()))
                            }
                            _ => bail!("invalid list index")
                        }
                    },
                    _ => bail!("index for getitem of list can only be int")
                },
                _ => bail!("index for getitem of list can only be int")
            }
        }
        __bool__(state, _self) {
            match &*_self.clone().borrow() {
                WdAny::Obj(o) => match &o.buildin {
                    BuildIn::Tuple(t) => Ok(match t.len() != 0 {
                        true => utils::get_buildin_var("true", state)?,
                        false => utils::get_buildin_var("false", state)?,
                    }),
                    _ => unreachable!()
                },
                _ => unreachable!(),
            }
        }
        __string__(state, _self) {
            match &*_self.clone().borrow() {
                WdAny::Obj(o) => match &o.buildin {
                    BuildIn::Tuple(t) => {
                        let items = t.iter()
                            .map(|e| match utils::convert2string(e.clone(), state.clone()) {
                                    Ok(s) => s,
                                    Err(_) => "<?>".to_string(),
                                }
                            )
                            .collect::<Vec<String>>().join(", ");
                        Ok(obj_string::build_string(&format!("[{}]", items), state.clone()))
                    },
                    _ => unreachable!()
                },
                _ => unreachable!(),
            }
        }
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state, 
        "list", 
        res.clone()
    )?;
    Ok(res)
}

pub fn buildin_list_post(type_obj: Any, state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?,
        "__name__".to_string() => obj_string::build_string("list", state.clone())
    };
    for (k, v) in attrs.into_iter() {
        utils::set_attr(type_obj.clone(), &k, v)?;
    }
    Ok(())
}

pub fn build_list(arr: &VecDeque<Any>, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::Tuple(arr.clone()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("list", state.clone()).unwrap()
        }
    })))
}

pub fn any2vecdeque(x: Any) -> Option<VecDeque<Any>> {
    match &*x.clone().borrow() {
        WdAny::Obj(o) => match &o.buildin {
            BuildIn::Tuple(v) => Some(v.clone()),
            _ => None
        },
        _ => None,
    }
}