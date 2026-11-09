fn main() {
    let a = A{};
    let _c: C = chaincast::chaincast!(
        a;
        B,
        C
    );
}

struct A;
struct B;
struct C;

impl From<A> for B {
    fn from(_a: A) -> B {
        B {}
    }
}

impl From<B> for C {
    fn from(_a: B) -> C {
        C {}
    }
}