const SIZE: usize = 16;

fn bar(count: u64, prev: &mut [u8; SIZE]) -> u8 {
    let mut next = [0; SIZE];
    next[0] = 0xab;
    if count == 0 {
        return prev[0];
    }
    bar(count - 1, &mut next)
}

fn setup() -> Vec<Vec<u8>> {
    let mut vecs = vec![];
    let mut current_max = 0;
    loop {
        let mut new_vec: Vec<u8> = Vec::new();
        if let Err(_) = new_vec.try_reserve_exact(4096) {
            println!("current highest address {:x}", current_max);
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
            println!("current highest address {:x}", current_max);
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

    let mut init = [0_u8; SIZE];
    let count = (1024 * 1024) / (2 * SIZE as u64) + 2;
    bar(count, &mut init);

    println!("checking vecs");
    for v in vecs {
        for b in v {
            assert_eq!(b, 0, "Vector has non-zero value 0x{:x}", b);
        }
    }
}
