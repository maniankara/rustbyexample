struct A; // concrete A

struct S(A); // concrete S(A)

struct SGen<T>(T); // generic SGen

fn gen_spec_t(_s: SGen<A>) {} // not generic

fn gen_spec_i32(_s: SGen<i32>) {} // not generic

fn gen<T>(_s: SGen<T>) {} // valid generic over T

fn main() {

    gen::<char>(SGen('a')) ; // explicit

    gen(SGen('c')) // implicit
}


