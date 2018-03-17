# Number Theory using Rust's type system

Rust does not have dependent types, or GADTs like Haskell, but with a few tricks, we can use Rust's type system to emulate an Idris-like number system.

This is not meant to be a proper introduction into Number Theory or Rust or Rust's type system, so at places, I'll use *non proper mathematical* notation and methods to explain.

## Precursor

Before you read this, some basic knowledge of number theory, as well as knowledge of Rust's trait system and generics would be necessary.

- [Number Theory - Wikipedia](https://en.wikipedia.org/wiki/Number_theory)
- [Generics - TRPL](https://doc.rust-lang.org/book/second-edition/ch10-00-generics.html)
- [Traits - TRPL](https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html)

## Values

First, we need a type to represent the values.

```rust
#[derive(Debug, Clone, Copy)]
struct Z;
#[derive(Debug, Clone, Copy)]
struct S<T>(T);
```

`Z` and `S(T)` are only types, they cannot represent any actual values. To do this, we are going to need to connect them to actual values.

```rust
#[derive(Debug, Clone, Copy)]
struct Nat<T>(T);

const Zero: Nat<Z> = Nat(Z);
```

Here, we have an actual value, `Zero`. This is the number `0`, and the basis where we are going to build upon for our system. Now lets add in the rest of the positive integers.

```rust
fn Succ<A>(d: Nat<A>) -> Nat<S<A>> {
    Nat(S(d.0))
}
```

This function allows us to work out the next value. Let's test this out.

```rust
fn main() {
    println!("{:?}", Succ(Zero));
    println!("{:?}", Succ(Succ(Zero)));
}
```

**Output:**

```nil
Nat(S(Z))
Nat(S(S(Z)))
```

It works! The successor to `Zero` is `Nat(S(Z))`, which represents the number after zero, also known as `One`. `One` has the type `Nat<S<Z>>`. The next line shows that this works for the successor of `One`, which gives us `Nat(S(Z))`. Note that these are not actual values, and simply just different types. Converting these to a Rust unsigned integer is possible, but not the point of the exercise.

Simply having the set of Natural numbers is rather pointless without the ability to do anything with them. We need to give them the ability to do arithmetic.

## Arithmetic

### Addition

So, what should `Nat<N> + Nat<M>` be? It is going to be a natural number, but how would we represent that value? Lets start by defining addition here.

Anything plus `Z` is itself. Lets start by implementing this. Rust already has a trait for addition, [`std::ops::Add`](https://doc.rust-lang.org/std/ops/trait.Add.html). Lets use this for our numbers.

```rust
use std::ops::Add;

impl<M> Add<M> for Z {
    type Output = M;

    fn add(self, rhs: M) -> M {
        rhs
    }
}
```

Given the base case of

```rust
// 0 + a = a
Z + A = A
```

we can extend this by changing the left hand side of the equation. Say if added 1 to the left hand side by changing it to its successor

```rust
// (0 + 1) + a = ???
S<Z> + A = ???
```

When we simplify this down algebraically, we get `a + 1` on the right hand side. Now, as we have defined `S<T>` simply to mean the successor to `T`, which happens to be what we want in this case.

```rust
// (0 + 1) + a = a + 1
Nat<S<Z>> + Nat<A> = Nat<S<A>>
```

Now lets think about this generically. Say now the left hand side is `S<N>`, what would we do?

```rust
// (0 + 1 + 1 + ..) + a = ???
S<N> + A = ???
```

We can look at this through induction. As we know the answers to `Z + A` and `S<Z> + A`, the pattern would imply that the answer to this is

```rust
S<N> + A = S<A+N>
```

Lets try this out and see whether it is correct. This means that we will need to implement this generically for all `T`, as we do not know what `T` is.

```rust
impl<M, N, O> Add<M> for S<N>
where
    N: Add<M, Output = O>,
{
    // O = A + N
    type Output = S<O>;

    fn add(self, rhs: M) -> S<O> {
        S(self.0 + rhs)
    }
}
```

Time to test this out and see whether our implementation is correct.

```rust
fn main() {
    // 1 + 1
    println!("{:?}", S(Z) + S(Z));
    // 2 + 2
    println!("{:?}", S(S(Z)) + S(S(Z)));
    // 2 + 3
    println!("{:?}", S(S(Z)) + S(S(S(Z))));
}
```

**Output:**

```nil
S(S(Z))
S(S(S(S(Z))))
S(S(S(S(S(Z)))))
```

We get the correct answers, `2`, `4` and `5` respectively.

To finish it off, we need to define this for `Nat<N>`, which we can do like so:

```rust
impl<M, N, O> Add<Nat<N>> for Nat<M>
where
    M: Add<N, Output = O>,
{
    type Output = Nat<O>;

    fn add(self, rhs: Nat<N>) -> Nat<O> {
        Nat(self.0 + rhs.0)
    }
}
```

This just takes the inner values to the two `Nat`s and adds them together, wrapping it back into a `Nat`. Testing this out:

```rust
fn main() {
    println!("{:?}", Succ(Succ(Zero)) + Succ(Zero))
}
```

**Output:**

```nil
Nat(S(S(S(Z))))
```

#### Proper Explanation

Each `S(N)` can be expanded into `S(S(T))` recursively, until it reaches `S(...S(Z))`. This means that if there is `n` 'layers' of `S`, we just need to add `n` to the right hand side, which is the `n`th successor of the right hand side.

### Multiplication

Now that we have addition, we can define repeated addition, also known as multiplication. Once again, we are going to start with a base case using `Z`. `Z` multiplied by anything will give `Z`. Lets implement this

```rust
use std::ops::Mul;

impl<M> Mul<M> for Z {
    type Output = Z;

    fn mul(self, _rhs: M) -> Z {
        Z
    }
}
```

Which shows the following:

```rust
Z * S<M> = Z
```

Now we need to change the left hand side to `S<N>` to be able to multiply by numbers that are not `Z`. To do this, we can simplify the multiplication like so:

```nil
a * b = b + (a - 1)(b)
```

which we can rearrage using our `S<N>` notation as:

```nil
S(a) * b = b + a(b)
```

We can then implement it in Rust similar to how we implemented `Add`.

```rust
impl<M, N, O, P> Mul<M> for S<N>
where
    N: Mul<M, Output = O>,
    M: Add<O, Output = P> + Copy,
{
    type Output = P;

    fn mul(self, rhs: M) -> P {
        rhs.clone() + (self.0 * rhs)
    }
}
```

Once again, we will need to implement `Mul` for `Nat`, as we have only currently implemented `Mul` for the inner types.

```rust
impl<M, N, O> Mul<Nat<N>> for Nat<M>
where
    M: Mul<N, Output = O>,
{
    type Output = Nat<O>;

    fn mul(self, rhs: Nat<N>) -> Nat<O> {
        Nat(self.0 * rhs.0)
    }
}
```

Testing this out:

```rust
fn main() {
    // 1 * 1
    println!("{:?}", S(Z) * S(Z));
    // 2 * 2
    println!("{:?}", S(S(Z)) * S(S(Z)));
    // 2 * 3
    println!("{:?}", S(S(Z)) * S(S(S(Z))));
}
```

**Output:**

```nil
S(Z)
S(S(S(S(Z))))
S(S(S(S(S(S(Z))))))
```

## Conclusion

Rust's type system allows for these simpler number theory examples to be expressed, but with more complex topics, like equality, it becomes much harder to express without currying. Is this an issue? For 99+% of the use cases of Rust, no.

We have implemented the foundations, upon which feel free to prove other things using it:

- Combination of Addition (`a + (b + c) = a + b + c`)
- Commutation of Addition (`a + b = b + a`)
- Combination of Multiplication(`a * (b * c) = a * b * c`)
- Commutation of Multiplication(`a * b = b * a`)

Using `Z, S(S(Z)) ...` as the even numbers, and `S(Z), S(S(S(Z))) ...` as the odds, you can try and prove these:

- Odd + Even = Odd
- Odd + Odd = Even
- Even + Even = Even
- Odd * Even = Even
- Odd * Odd = Odd