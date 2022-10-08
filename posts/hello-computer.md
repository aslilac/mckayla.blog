---
title: Hello, computer!
author: Kayla Washburn
published: 2022.10.7
---

# Header

This is an example blog post, with a bunch of source code in different languages
and usage of a bunch of Markdown features.

lmao, this parser doesn't support comments

<!--
<details>
<summary>August is nice</summary>
<p>Soft puppy</p>
</details>
-->

> Quote

- Bulleted list
- Bulleted list

1. Numbered list
1. Numbered list

```rust
fn main() {
	println!("hello, computer!");
}
```

```gleam
import gleam/io

fn main() {
	io.println("hello, computer!")
}
```

```zig
const std = @import("std");

fn main() {
	std.log.info("hello, computer!");
}
```

```swift
print("hello, computer!")
```

```cpp
#include <iostream>
using std::cout;

auto main() -> int {
	cout << "hello, computer!\n";
}
```

```go
import (
	"fmt"
)

func main() {
	fmt.Println("hello, computer!")
}
```

```haskell
main :: IO ()
main = do
  putstrln "hello, computer!"
```

```elm
module App exposing (main)

import Browser

main : Program () Model Update
main =
    Browser.sandbox
        { init = init
        , update = update
        , view = view
        }
```
