use std::slice;

#[allow(unused)]
unsafe fn pointers(){
    let mut number:i64 =5;

    let r1 =&number as *const i64;
    let r2 =&mut number as *mut i64;

    //let address =0x012345usize;
    //let r = address as *const i64;

    unsafe{
        println!("r1 equal: {}",*r1);
        println!("r1 equal: {}",*r2);
    }
}

#[allow(unused)]
fn split_as_mut(slice: &mut [i64],mid: usize)->(&mut [i64], &mut [i64]){
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
unsafe{
(
slice::from_raw_parts_mut(ptr, mid),
slice::from_raw_parts_mut(ptr.add(mid), len-mid)
)
}
}

fn main() {
    unsafe{
        pointers();
    }
}
