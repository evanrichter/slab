#![no_main]
use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
enum Op {
    Insert(u8),
    Remove(u16),
    Compact,
    Clear,
    Len,
    Get(u16),
}

fuzz_target!(|data: (u8, Vec<Op>)| {
    let (cap, ops) = data;
    let mut slab = slab::Slab::with_capacity(cap as usize);
    for op in ops {
        match op {
            Op::Insert(v) => drop(slab.insert(v)),
            Op::Remove(i) => drop(slab.try_remove(i as usize)),
            Op::Compact => slab.compact(|_, _, _| true),
            Op::Clear => slab.clear(),
            Op::Len => drop(slab.len()),
            Op::Get(i) => drop(slab.get(i as usize)),
        }
    }
});
