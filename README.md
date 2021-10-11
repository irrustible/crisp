# crisp

A simple, securable, embeddable lisp interpreter for rust programs.

## Status

Incredibly pre-alpha, is not fully implemented, doesn't work yet.

## Examples

```crisp
(defn fac [n]
  (if (< n 2)
    1
    (n * (fac (- n 1)))))
(defn sum [... ns]
  (apply + ns))
```

## Design

Crisp is initially designed to be a simple interpreter.

### Vexes

Fexprs are a first-class alternative to macros. The idea is quite
simple, they're like functions that take their arguments unevaluated
and the calling environment. They can choose to evaluate their
arguments in the calling environment or not.

Fexprs were conceived fairly early on but didn't really stick because
dynamic scope was the predominant paradigm of the day. These days the
dominant paradigm is lexical scope and it turns out they work quite
well.

Fexprs are more powerful than macros and remove both the
macroexpansion step and the need for hygiene (we have lexical scope,
which is better!). Because there is no generating code per se, code
always has a direct correspondence to what was typed.

## Notes

Built-in primitives:

* `fx` - create an anonymos f-expression
* `fn` - create an anonymos function
* `define` - define a value in the current module
* `defx` - define an f-expression in the current module
* `defn` - define a function in the current module
* `spawn` - start a new greenthread

## Copyright and License

Copyright (c) 2021 James Laver, crisp contributors

[Licensed](LICENSE) under Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0),
with LLVM Exceptions (https://spdx.org/licenses/LLVM-exception.html).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

