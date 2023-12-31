# WhileDB
rust implementation of [While-DB](https://github.com/HellOwhatAs/While-DB), the programming language used in SJTU CS2612 (2022 Fall) Attachment 1014

## TODO
- [x] find a way to omit `;` after `}`
  currently unable to add `;`, however
- [x] add function
- [x] make any expression callable (currently only `ident` callable)
- [x] add class and method
- [x] add `[` `]` to represent array and getitem
- [x] add string
- [x] add interpreter or compiler
  > any object (including the program state) can be represented as `Any`, pointer to `WdAny`
  > `WdAny` is either `Object` or `Function`
  > `Object` has 2 fields `buildin` (store buildin types or `Not`) and `attrs`(a string to `Any` map)
  > `Function` is either build-in or defined
  >
  > state store 
  > - types (object with `"__name__"` => typename, and `"__method_name__"` => method function)
  > - instances (object with `"__type__"` => type object)
  > - functions ...

  > state = `{ <locals>, "..": {<nonlocals>} }` 😎, only do this in DefinedFunction (buildin function do not have locals)

  > call an expression
  > - expr is a `expr0.ident` and expr is a instance (`__type__` is not `type`), args = `[expr0, *args]`
  >   plan to check this via add flags on return value of `eval`
  >   `pub fn eval(expr: Rc<Expr>, state: Any) -> Result<(Any, Option<Any>)>` etc. 
  >   > plan to check `break`, `continue`, `return` the same way.
  > - expr is a `WdAny::Function`, original args
  > - expr if a `Object`
  >   - with `__init__` self-attr, args = `[new_obj (with __type__ => this expr), *args]`
  >   - with `__call__` self-attr, original args
  >   - with `__call__` attr found from `__type__` attr, args = `[this expr, *args]`
  

## Future Work
- [x] python binding: [WhileDB.rs](https://github.com/HellOwhatAs/WhileDB.rs)
- [ ] wasm