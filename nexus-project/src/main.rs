#![no_std]
#![no_main]

fn tri(n: u32) -> u32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => tri(n - 1) + tri(n - 2) + tri(n - 3),
    }
}

#[nexus_rt::main]
fn main() {
    let n = 7;
    let result = tri(n);
    assert_eq!(result, 24);
}
