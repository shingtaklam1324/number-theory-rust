#![feature(conservative_impl_trait)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[derive(Debug, Clone, Copy)]
struct Z;
#[derive(Debug, Clone, Copy)]
struct S<T>(T);
#[derive(Debug, Clone, Copy)]
struct Nat<A>(A);

const Zero: Nat<Z> = Nat(Z);

fn Succ<A>(d: Nat<A>) -> Nat<S<A>> {
    Nat(S(d.0))
}

// trait Plus<T> {
//     type Output;

//     fn plus(self, rhs: T) -> Self::Output;
// }

// trait Times<T> {
//     type Output;

//     fn times(self, rhs: T) -> Self::Output;
// }

// impl<M> Plus<M> for Z {
//     type Output = M;

//     fn plus(self, rhs: M) -> M {
//         rhs
//     }
// }

impl<M> Add<M> for Z {
    type Output = M;

    fn add(self, rhs: M) -> M {
        rhs
    }
}

// impl<M, N, O> Plus<M> for S<N>
// where
//     N: Plus<M, Output = O>,
// {
//     type Output = S<O>;

//     fn plus(self, rhs: M) -> S<O> {
//         S(self.0.plus(rhs))
//     }
// }

impl<M, N, O> Add<M> for S<N>
where
    N: Add<M, Output = O>,
{
    type Output = S<O>;

    fn add(self, rhs: M) -> S<O> {
        S(self.0 + rhs)
    }
}

// impl<M> Times<M> for Z {
//     type Output = Z;

//     fn times(self, _rhs: M) -> Z {
//         Z
//     }
// }

impl<M> Mul<M> for Z {
    type Output = Z;

    fn mul(self, _rhs: M) -> Z {
        Z
    }
}

// impl<M, N, O, P> Times<M> for S<N>
// where
//     N: Times<M, Output = O>,
//     M: Plus<O, Output = P> + Clone,
// {
//     type Output = P;

//     fn times(self, rhs: M) -> P {
//         rhs.clone().plus(self.0.times(rhs))
//     }
// }

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

use std::ops::{Add, Mul};

impl<M, N, O> Add<Nat<N>> for Nat<M>
where
    M: Add<N, Output = O>,
{
    type Output = Nat<O>;

    fn add(self, rhs: Nat<N>) -> Nat<O> {
        Nat(self.0 + rhs.0)
    }
}

impl<M, N, O> Mul<Nat<N>> for Nat<M>
where
    M: Mul<N, Output = O>,
{
    type Output = Nat<O>;

    fn mul(self, rhs: Nat<N>) -> Nat<O> {
        Nat(self.0 * rhs.0)
    }
}

// fn curry<A: 'static, B: 'static, C: 'static>(f: fn(A, B) -> C, a: A) -> Box<FnOnce(B) -> C> {
//     Box::new(move |b: B| f(a, b))
// }

// #[derive(Clone, Copy, Debug)]
// struct Equal<A, B>(A, B);

// type EqualZ = Equal<Z, Z>;
// type EqualS<A, B> = Equal<S<A>, S<B>>;

// fn equal_s<A, B>(a: A, b: B) -> EqualS<A, B> {
//     Equal(S(a), S(b))
// }

// trait Refl<A> {
//     type Output;

//     fn refl(self) -> Self::Output;
// }

// impl Refl<Z> for Z {
//     type Output = Box<FnOnce(Z) -> EqualZ>;

//     fn refl(self) -> Self::Output {
//         curry(Equal, self)
//     }
// }

// impl<T: 'static, A: 'static, U: 'static> Refl<A> for S<T>
// where
//     T: Refl<A, Output = U>,
// {
//     type Output = Box<FnOnce(A) -> Equal<S<U>, S<A>>>;

//     fn refl(self) -> Self::Output {
//         curry(equal_s, self.0.refl())
//     }
// }

fn main() {
    println!("{:?}", S(Z) * S(Z));
    println!("{:?}", S(S(Z)) * S(S(Z)));
    println!("{:?}", S(S(Z)) * S(S(S(Z))));
}
