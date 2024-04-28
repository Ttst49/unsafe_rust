#[allow(unused)]
fn pointers(){
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

fn main() {
    pointers();
}
