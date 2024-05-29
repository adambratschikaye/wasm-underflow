use candid::{CandidType, Decode, Deserialize, Encode};
// use dlmalloc::GlobalDlmalloc;
use serde::Serialize;

// #[global_allocator]
// static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;

#[derive(CandidType, Deserialize, Serialize)]
enum Tree {
    Leaf,
    SubTree(Vec<Tree>),
}

fn make_tree() -> Tree {
    let n = 4096;

    let mut tree = Tree::Leaf;
    for _ in 0..n {
        tree = Tree::SubTree(vec![tree]);
    }

    tree
}

#[no_mangle]
pub fn candid_tree() {
    let tree = make_tree();

    let bytes = Encode!(&tree).unwrap();
    Decode!(&bytes, Tree).unwrap();
}

#[no_mangle]
pub fn json_tree() {
    let tree = make_tree();

    let bytes = serde_json::to_vec(&tree).unwrap();
    let mut deserializer = serde_json::Deserializer::from_slice(&bytes);
    deserializer.disable_recursion_limit();
    let _result: Tree = serde::de::Deserialize::deserialize(&mut deserializer).unwrap();
}

struct Bar {
    inner: [u8; 16],
}

impl Bar {
    fn new() -> Self {
        Bar { inner: [0xab; 16] }
    }
}

fn bar<'a>(count: u64, prev: &mut Bar) -> u8 {
    if count == 0 {
        return prev.inner[0];
    }
    let mut next = Bar::new();
    bar(count - 1, &mut next)
}

#[no_mangle]
pub fn foo() {
    let mut init = Bar::new();
    let count = (1024 * 1024) / 16 + 2;
    bar(count, &mut init);
    // let mut v = vec![];
    // for _ in 0..1024 * 1024 {
    //     println!(
    //         "address: {:p}, length: {:x}, capacity: {:x}, end: {:x}",
    //         v.as_ptr(),
    //         v.len(),
    //         v.capacity(),
    //         v.as_ptr() as usize + v.capacity(),
    //     );
    //     std::io::Write::write_all(&mut v, &[0xab_u8]).unwrap();
    // }
}

#[test]
fn test_json() {
    json_tree()
}
