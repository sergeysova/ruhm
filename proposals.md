# Proposals

## Trait

Input:

```rust
trait Example {
  fn foo(self, add: i32) -> i32 {
    self.value + add + 1
  }
}

fn main() {
  /* impl Example */
  some.foo(2)
}
```

Output:

```js
const trait_Example = {
  foo(self, add) {
    return self.value + add + 1
  }
}

function main() {
  /* impl Example */
  return trait_Example.foo(some, 2)
}
```

https://www.measurethat.net/Benchmarks/Show/3103/3/direct-call-vs-bind-vs-call-vs-apply-vs-self

Maybe compile to `.call` expression:

```js
const trait_Example = {
  foo(add) {
    return this.value + add + 1
  }
}

function main() {
  /* impl Example */
  return trait_Example.foo.call(some, 2)
}
```


## Struct

Input:

```rust
struct Example {
  pub foo: i32,
  bar: String,
}

impl Example {
  fn new(foo: i32, bar: String) -> Example {
    Example { foo, bar }
  }

  fn bar(self) -> String {
    self.bar
  }
}

impl Drop for Example {
  fn drop(self) {
    println!("drop: {}", self.foo)
  }
}

fn main() {
  let foo = Example::new(1, "10");
  foo.bar()
}
```

Output:

```js
const struct_Example = {
  new(foo, bar) {
    return { foo: Number(foo), bar: String(bar) }
  },
  bar() {
    return this.bar
  }
}

const drop_for_Example = {
  drop() {
    console.log("drop: %s", this.foo)
  }
}

function main() {
  const foo = struct_Example.new(1, "10")
  struct_Example.bar.call(foo)

  // drops
  drop_for_Example(foo)
  foo = null
}
```

Maybe remove type cast?


## Symbol for struct

Maybe add symbol for each struct invocation?

It's usable for dynamic dispatch. https://doc.rust-lang.org/1.0.0-beta/book/static-and-dynamic-dispatch.html

Input:

```rust
struct Example {
  foo: i32,
  bar: u16,
}

impl Example {
  fn new(foo: i32, bar: u16) -> Example {
    Example { foo, bar }
  }
}
```

Output:

```js
// runtime
const type = Symbol('@@type')

// user code
const struct_Example_id = Symbol('Example')

const struct_Example = {
  new(foo, bar) {
    return { [type]: struct_Example_id, foo, bar }
  }
}
```


