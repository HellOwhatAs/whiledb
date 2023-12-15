use crate::interpreter::*;

pub fn exec(ast: Rc<Cmd>, state: Any) -> Result<(), String> {
    match ast.as_ref() {
        Cmd::Asgn(e1, e2) => {
            match e1.as_ref() {
                Expr::Var(s) => {
                    let v2 = eval(e2.clone(), state.clone())?;
                    utils::set_attr(state, s, v2)
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
        Cmd::Expr(e) => {
            eval(e.clone(), state)?;
            Ok(())
        },
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
        Expr::Tuple(es) => {
            let mut vs = VecDeque::new();
            for e in es.iter() {
                vs.push_back(eval(e.clone(), state.clone())?);
            }
            Ok(Rc::new(RefCell::new(WdAny::Obj(Object {
                buildin: BuildIn::Tuple(vs),
                attrs: HashMap::new()
            }))))
        },
        Expr::Var(s) => utils::get_var(s, state.clone()),
        Expr::BinOp(op, e1, e2) => {
            let (v1, v2) = (eval(e1.clone(), state.clone())?,eval(e2.clone(), state.clone())?);
            match (utils::get_attr(v1.clone(), &format!("__{}__", op)), utils::get_attr(v2.clone(), &format!("__r{}__", op))) {
                (Some(f), _) => utils::call(f, &VecDeque::from([v1.clone(), v2.clone()]), state),
                (None, Some(rf)) => utils::call(rf, &VecDeque::from([v2.clone(), v1.clone()]), state),
                _ => Err(format!("Cannot {} between {:?} and {:?}", op, v1, v2))
            }
        },
        Expr::UnOp(_, _) => todo!(),
        Expr::Call(e1, e2) => {
            let (v1, v2) = (eval(e1.clone(), state.clone())?, eval(e2.clone(), state.clone())?);
            match &*v2.clone().borrow() {
                WdAny::Obj(o) => {
                    match &o.buildin {
                        BuildIn::Tuple(args) => utils::call(v1, args, state),
                        _ => unreachable!()
                    }
                },
                _ => unreachable!(),
            }
        },
        Expr::GetItem(_, _) => todo!(),
        Expr::GetAttr(_, _) => todo!(),
    }
}