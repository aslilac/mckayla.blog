---
title: Why are circular references in Rust so hard?
author: Kayla Washburn
status: draft
---

If you just came here for the literal answer to the question, "how can I make a circular
reference in Rust?", I won't waste your time. Here's one way you can do it:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Friend {
	name: String,
	mutual: Option<Rc<RefCell<Friend>>>,
}

impl Friend {
	fn named(name: &str) -> Self {
		Friend { name: name.to_string(), mutual: None }
	}

	// `Rc` handles reference counting, `RefCell` allows us to obtain a mutable reference
	fn into_rc(self) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(self))
	}
}

fn main() {
	let lilac = Friend::named("lilac").into_rc();
	let rattard = Friend::named("rattard").into_rc();

	lilac.borrow_mut().mutual = Some(rattard.clone());
	rattard.borrow_mut().mutual = Some(lilac.clone());
}
```

You're still here? You _really_ wanna know how this works? Okay, take a deep breath, and
we can get started.
