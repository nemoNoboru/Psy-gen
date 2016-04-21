extern crate rand;
extern crate libc;

use libc::int32_t;

pub struct Psygen {
    used: Vec<i32>,
    max: usize,
}

impl Psygen {
    fn new(max:usize) -> Psygen {
        Psygen{ used: vec![0;max+1], max:max }
    }

    fn add_used(&mut self, used:i32){
        self.used[(used as usize)] = 1;
    }

    fn next(&mut self) -> i32 {
        if !self.free() {
            return -1;
        }
        loop {
            let t:usize = rand::random::<usize>() % (self.max + 1);
            //println!("probando el {}", t );
            if self.used[t] == 0 {
                self.used[t] = 1;
                return t as i32;
            }
        }
    }

    fn free(&self) -> bool {
        let mut t:usize = 0;
        while t < (self.used.len() - 1) {
            if self.used[t] == 0 {
                return true;
            }
            t += 1;
        }
        return false;
    }
}

#[no_mangle]
pub extern fn newPsygen(i:int32_t) -> *mut Psygen {
    Box::into_raw(Box::new(Psygen::new((i as usize))))
}

#[no_mangle]
pub extern fn next(p: *mut Psygen) -> int32_t {
    let mut ptr = unsafe {
        assert!(!p.is_null());
        &mut *p
    };
    ptr.next() as int32_t
}

#[no_mangle]
pub extern fn psygen_free(ptr: *mut Psygen) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}
