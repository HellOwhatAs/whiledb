use crate::interpreter::*;

pub fn exec(ast: Rc<Cmd>, state: Any) -> Result<(), String> {
    match ast.as_ref() {
        Cmd::Asgn(e1, e2) => {
            match e1.as_ref() {
                Expr::Var(s) => {
                    let v2 = eval(e2.clone(), state.clone())?;
                    match &mut *state.borrow_mut() {
                        WdAny::Obj(o) => {
                            o.attrs.insert(s.clone(), v2);
                            Ok(())
                        },
                        _ => unreachable!(),
                    }
                },
                _ => {
                    let v1 = eval(e1.clone(), state.clone())?;
                    let v2 = eval(e2.clone(), state)?;
                    if Rc::strong_count(&v1) == 1 {
                        return Err(format!("Cannot assign to {:?}", e1));
                    }
                    let _ = std::mem::replace(&mut v1.borrow_mut(), v2.borrow_mut());
                    Ok(())
                }
            }
        },
        Cmd::Seq(cs) => {
            for c in cs.iter() {
                exec(c.clone(), state.clone())?;
            }
            Ok(())
        },
        Cmd::If(_, _, _) => todo!(),
        Cmd::While(_, _) => todo!(),
        Cmd::Expr(_) => todo!(),
        Cmd::Continue => todo!(),
        Cmd::Break => todo!(),
        Cmd::Func(_, _, _) => todo!(),
        Cmd::Class(_, _) => todo!(),
        Cmd::Return(_) => todo!(),
        Cmd::Nop => Ok(()),
    }
}

pub fn eval(expr: Rc<Expr>, state: Any) -> Result<Any, String> {
    match expr.as_ref() {
        Expr::ConstInt(s) => Ok(obj_int::build_int(s, state.clone())),
        Expr::ConstFloat(_) => todo!(),
        Expr::ConstString(_) => todo!(),
        Expr::Tuple(_) => todo!(),
        Expr::Var(s) => {
            match &*state.clone().borrow() {
                WdAny::Obj(o) => {
                    match o.attrs.get(s) {
                        Some(e) => Ok(e.clone()),
                        None => Err(format!("Using undefined variable `{}`", s)),
                    }
                },
                _ => unreachable!(),
            }
        },
        Expr::BinOp(op, e1, e2) => {
            let (v1, v2) = (eval(e1.clone(), state.clone())?,eval(e2.clone(), state.clone())?);
            match (utils::get_attr(v1.clone(), &format!("__{}__", op)), utils::get_attr(v2.clone(), &format!("__r{}__", op))) {
                (Some(f), _) => utils::call(f, VecDeque::from([v1.clone(), v2.clone()]), state),
                (None, Some(rf)) => utils::call(rf, VecDeque::from([v2.clone(), v1.clone()]), state),
                _ => Err(format!("Cannot {} between {:?} and {:?}", op, v1, v2))
            }
        },
        Expr::UnOp(_, _) => todo!(),
        Expr::Call(_, _) => todo!(),
        Expr::GetItem(_, _) => todo!(),
        Expr::GetAttr(_, _) => todo!(),
    }
}