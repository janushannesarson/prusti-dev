#![feature(nll)]
#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate prusti_contracts;

struct Point {
    x: Box<u32>,
    y: Box<u32>,
}

#[ensures="*result == old(*a) + old(b)"]
fn add(a: Box<u32>, b: u32) -> Box<u32> { box (*a + b) }

#[ensures="old(*p.x) == *result.x + old(s)"]
#[ensures="old(*p.y) == *result.y"]
fn shift_x(p: Point, s: u32) -> Point {
    let mut pp = p;
    let x = pp.x;     // p.x is moved out.
    pp.x = add(x, s); // x is moved into add,
                      // result moved into p.x
    pp
}

fn main(){}
