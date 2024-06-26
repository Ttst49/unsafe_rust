use std::{i64, slice};

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

extern "C"{
    fn abs(input: i64)->i64;
}

#[allow(unused)]
fn use_abs_from_c(x:i64){
    unsafe{
        println!("That's the representation of {} as an absolute in C: {}",x,abs(x))
    }
}


#[allow(unused)]
static HELLO_WORLD: &str ="Hello, world!";
#[allow(unused)]
static mut COUNTER: u64 = 0;

#[allow(unused)]
fn add_to_counter(value:u64) {
    unsafe{
        COUNTER+=value;
    }
}

#[allow(unused)]
fn use_unsafe_with_static(x:u64) {
    add_to_counter(x);
    unsafe{
        println!("COUNTER : {}",COUNTER)
    }
}

unsafe trait Foo {
    //omg that's an unsafe trait
}

unsafe impl Foo for i64 {
    //we're defining our methods here
}

fn main() {
    use_unsafe_with_static(3);
}
