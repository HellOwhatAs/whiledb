use crate::interpreter::*;

/// Result<(continue, break, return)>
pub fn exec(ast: Rc<Cmd>, state: Any) -> Result<(bool, bool, Option<Any>)> {
    match ast.as_ref() {
        Cmd::Asgn(e1, e2) => {
            match e1.as_ref() {
                Expr::Var(s) => {
                    let (v2, _) = eval(e2.clone(), state.clone())?;
                    utils::set_attr(state, s, v2)?;
                    Ok((false, false, None))
                },
                _ => {
                    let (v1, _) = eval(e1.clone(), state.clone())?;
                    let (v2, _) = eval(e2.clone(), state)?;
                    if Rc::strong_count(&v1) == 1 {
                        bail!("Cannot assign to {:?}", e1)
                    }
                    let _ = std::mem::replace(&mut v1.borrow_mut(), v2.borrow_mut());
                    Ok((false, false, None))
                }
            }
        },
        Cmd::Seq(cs) => {
            for c in cs.iter() {
                let (cont, brk, ret) = exec(c.clone(), state.clone())?;
                if cont || brk || ret.is_some() {
                    return Ok((cont, brk, ret))
                }
            }
            Ok((false, false, None))
        },
        Cmd::If(e, c1, c2) => {
            let (v, _) = eval(e.clone(), state.clone())?;
            match utils::get_attr(v.clone(), "__bool__") {
                Some(f) => match obj_bool::any2bool(utils::call(f, VecDeque::from([v.clone()]), state.clone())?) {
                    Some(b) => if b { exec(c1.clone(), state) } else { exec(c2.clone(), state) },
                    None => unreachable!(),
                },
                None => bail!("if condition v cannot convert to bools")
            }
        },
        Cmd::While(e, c) => {
            loop {
                let (v, _) = eval(e.clone(), state.clone())?;
                let b = match utils::get_attr(v.clone(), "__bool__") {
                    Some(f) => match obj_bool::any2bool(utils::call(f, VecDeque::from([v.clone()]), state.clone())?) {
                        Some(b) => b,
                        None => unreachable!(),
                    },
                    None => bail!("if condition v cannot convert to bools")
                };
                if !b {
                    break Ok((false, false, None));
                }
                let (cont, brk, ret) = exec(c.clone(), state.clone())?;
                match (cont, brk, ret.clone()) {
                    (false, false, Some(_)) => {
                        break Ok((false, false, ret));
                    }
                    (false, true, None) => {
                        break Ok((false, false, None));
                    }
                    (_, false, None) => {
                        continue;
                    }
                    _ => unreachable!()
                }
            }
        },
        Cmd::Expr(e) => {
            eval(e.clone(), state)?;
            Ok((false, false, None))
        },
        Cmd::Continue => Ok((true, false, None)),
        Cmd::Break => Ok((false, true, None)),
        Cmd::Func(_, _, _) => todo!(),
        Cmd::Class(_, _) => todo!(),
        Cmd::Return(e) => {
            let (v, _) = eval(e.clone(), state.clone())?;
            Ok((false, false, Some(v)))
        },
        Cmd::Nop => Ok((false, false, None)),
    }
}

pub fn eval(expr: Rc<Expr>, state: Any) -> Result<(Any, Option<Any>)> {
    match expr.as_ref() {
        Expr::ConstInt(s) => Ok((obj_int::build_int(s, state), None)),
        Expr::ConstFloat(_) => todo!(),
        Expr::ConstString(s) => Ok((obj_string::build_string_raw(s, state)?, None)),
        Expr::Tuple(es) => {
            let mut vs = VecDeque::new();
            for e in es.iter() {
                vs.push_back(eval(e.clone(), state.clone())?.0);
            }
            Ok((Rc::new(RefCell::new(WdAny::Obj(Object {
                buildin: BuildIn::Tuple(vs),
                attrs: HashMap::new()
            }))), None))
        },
        Expr::Var(s) => Ok((utils::get_var(s, state.clone())?, None)),
        Expr::BinOp(op, e1, e2) => {
            let ((v1, _), (v2, _)) = (eval(e1.clone(), state.clone())?,eval(e2.clone(), state.clone())?);
            match (utils::get_attr(v1.clone(), &format!("__{}__", op)), utils::get_attr(v2.clone(), &format!("__r{}__", op))) {
                (Some(f), _) => Ok((utils::call(f, VecDeque::from([v1.clone(), v2.clone()]), state)?, None)),
                (None, Some(rf)) => Ok((utils::call(rf, VecDeque::from([v2.clone(), v1.clone()]), state)?, None)),
                _ => bail!("Cannot '{}' between 'v1' and 'v2'", op)
            }
        },
        Expr::UnOp(op, e) => {
            let (v, _) = eval(e.clone(), state.clone())?;
            match utils::get_attr(v.clone(), &format!("__{}__", op)) {
                Some(f) => Ok((utils::call(f, VecDeque::from([v.clone()]), state)?, None)),
                None => bail!("Cannot '{}' 'v'", op)
            }
        },
        Expr::Call(e1, e2) => {
            let ((v1, _self), (v2, _)) = (eval(e1.clone(), state.clone())?, eval(e2.clone(), state.clone())?);
            match &*v2.clone().borrow() {
                WdAny::Obj(o) => {
                    match &o.buildin {
                        BuildIn::Tuple(args) => match _self {
                            Some(_self) => {
                                let mut args = args.clone();
                                args.push_front(_self);
                                Ok((utils::call(v1, args.clone(), state)?, None))
                            },
                            None => Ok((utils::call(v1, args.clone(), state)?, None)),
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!(),
            }
        },
        Expr::GetItem(_, _) => todo!(),
        Expr::GetAttr(e, s) => {
            let (v, _) = eval(e.clone(), state.clone())?;
            match utils::get_self_attr(v.clone(), s) {
                Some(res) => Ok((res, None)),
                None => match utils::get_father_attr(v.clone(), s) {
                    Some(res) => Ok((res, Some(v))),
                    None => bail!("cannot get attr `{}`", s)
                }
            }
        },
    }
}