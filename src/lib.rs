use candid::{CandidType, Decode, Deserialize, Encode};
use serde::Serialize;

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

const SIZE: usize = 16;

struct Bar {
    inner: [u8; SIZE],
}

impl Bar {
    fn new() -> Self {
        Bar {
            inner: [0xab; SIZE],
        }
    }
}

fn bar<'a>(count: u64, prev: &mut Bar) -> u8 {
    let mut next = Bar::new();
    next.inner[0] = 0xab;
    if count == 0 {
        return prev.inner[0];
    }
    bar(count - 1, &mut next)
}

fn setup() -> Vec<Vec<u8>> {
    let mut vecs = vec![];
    let mut current_max = 0;
    loop {
        let mut new_vec: Vec<u8> = Vec::new();
        if let Err(_) = new_vec.try_reserve_exact(4096) {
            println!("current highest {:x}", current_max);
            break;
        }
        new_vec.extend_from_slice(&[0; 4096]);
        if new_vec.as_ptr() as usize > current_max {
            current_max = new_vec.as_ptr() as usize;
        }
        vecs.push(new_vec);
    }
    loop {
        let mut new_vec: Vec<u8> = Vec::new();
        if let Err(_) = new_vec.try_reserve_exact(1) {
            println!("current highest {:x}", current_max);
            break;
        }
        new_vec.extend_from_slice(&[0; 1]);
        if new_vec.as_ptr() as usize > current_max {
            current_max = new_vec.as_ptr() as usize;
        }
        vecs.push(new_vec);
    }
    vecs
}

#[no_mangle]
pub fn foo() {
    let vecs = setup();
    let mut init = Bar::new();
    let count = (1024 * 1024) / (SIZE as u64) + 2;
    bar(count, &mut init);
    println!("done");
    for v in vecs {
        for b in v {
            assert_eq!(b, 0);
        }
    }
}

#[test]
fn test_json() {
    json_tree()
}
