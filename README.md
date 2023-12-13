# WhileDB
rust implementation of [While-DB](https://github.com/HellOwhatAs/While-DB), the programming language used in SJTU CS2612 (2022 Fall) Attachment 1014

## TODO
- [x] find a way to omit `;` after `}`
  currently unable to add `;`, however
- [x] add function
- [x] make any expression callable (currently only `ident` callable)
- [x] add class and method
  ```
  // cmd_block
  class ident {
      fn_list
  }

  // expr
  expr.ident
  ```
- [ ] add `[` `]` to represent array and getitem
  ```
  // expr
  [expr_list]

  // expr
  expr[expr_list]
  ```
- [ ] add string
- [ ] add interpreter or compiler


## Future Work
- [x] python binding: [WhileDB.rs](https://github.com/HellOwhatAs/WhileDB.rs)
- [ ] wasm