const SIZE: usize = 16;

fn overflow_stack(count: u64, prev: &mut [[u8; SIZE]; SIZE]) -> u8 {
    let mut next = [[0; SIZE]; SIZE];
    next[0][0] = 0xab;
    // dummy has the same stack allocation as * const.
    // Useful for swapping between println without affecting offset.
    // printing stack location is useful to know the offset for preallocation.
    let _dummy = [0_i32; SIZE];
    // println!("Stack location at {:?}", next.as_ptr());
    if count == 0 {
        return prev[0][0];
    }
    overflow_stack(count - 1, &mut next)
}

fn allocate_zeros() -> Vec<Vec<u8>> {
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
pub fn main() {
    let vecs = allocate_zeros();
    // Stack preloader
    // Useful for aligning offset before underflow
    let _dummy = [0_i32; 368 * 500 + 364];

    let mut init = [[0_u8; SIZE]; SIZE];
    let count = 900;
    overflow_stack(count, &mut init);

    println!("checking vecs");
    for v in vecs {
        for b in v {
            assert_eq!(b, 0, "Vector has non-zero value 0x{:x}", b);
        }
    }
}
