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

## Copyright and License

Copyright (c) 2021 James Laver, crisp contributors

[Licensed](LICENSE) under Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0),
with LLVM Exceptions (https://spdx.org/licenses/LLVM-exception.html).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

