use std::mem::ManuallyDrop;
use std::rc::Rc;

static mut free_table: [Option<fn(&mut What) -> ()>; 4] = [None, None, None, None];
static mut to_string_table: [Option<fn(&PValue) -> String>; 4] = [None, None, None, None];

struct What;

impl Drop for What {
    fn drop(&mut self) {
        println!("it worked! (it did!) {:?}", self as *const Self);
    }
}

struct PValue {
    kind: u8,
    allocated: bool,
    data: PValueData,
}

impl PValue {
    const INVALID: u8 = 0;
    const INT: u8 = 1;
    
    fn invalid() -> PValue {
        PValue {
            kind: PValue::INVALID,
            allocated: false,
            data: PValueData {int: 0},
        }
    }
    
    fn int(value: u32) -> PValue {
        PValue {
            kind: PValue::INT,
            allocated: false,
            data: PValueData {int: value},
        }
    }
}

impl Drop for PValue {
    fn drop(&mut self) {
        if self.allocated {
            unsafe {
                println!("strong count is {}", Rc::strong_count(&self.data.ptr));
                let ptr = Rc::get_mut(&mut self.data.ptr).expect("aaaaaaa");
                free_table[self.kind as usize].expect("whif")(ptr);
                ManuallyDrop::drop(&mut self.data.ptr);
            }
        }
    }
}

union PValueData {
    ptr: ManuallyDrop<Rc::<What>>,
    int: u32,
}

impl PValue {
    pub fn to_string(&self) -> String {
        match self.kind {
            PValue::INVALID => {
                String::from("INVALID")
            },
            PValue::INT => unsafe {
                // accessing a union is unsafe...
                self.data.int.to_string()
            },
            kind => unsafe {
                // bad for multi threading
                // how do i lock the registration tables
                // not possible is it
                match to_string_table[kind as usize] {
                    Some(pf) => {
                        pf(self)
                    },
                    None => {
                        format!("<value of kind {kind}>")
                    }
                }
            }
        }
    }
    
    unsafe fn getptr(&self) -> *const What {
        Rc::as_ptr(&self.data.ptr)
    }
}

const A: u8 = 2;

unsafe fn register_a() {
    to_string_table[A as usize] = Some(|_v| String::from("a"));
}
    
fn make_a() -> PValue {
    PValue {
        kind: A,
        allocated: false,
        data: PValueData {int: 0},
    }
}

const B: u8 = 3;

unsafe fn register_b() {
    to_string_table[B as usize] = Some(|_v| String::from("b"));
    free_table[B as usize] = Some(|_v| {println!("it's getting freed!")});
}
    
fn make_b() -> PValue {
    PValue {
        kind: B,
        allocated: true,
        data: PValueData {ptr: ManuallyDrop::new(What.into())},
    }
}
/*
const C: u8 = 3;

unsafe fn register_c() {
    to_string_table[C as usize] = Some(|v| format!("c<{}>", unsafe{*(v.getptr() as *const () as *const u32)}));
    free_table[C as usize] = Some(|_v| {println!("it's getting freed!")});
}
    
fn make_c(n: u32) -> PValue {
    let ptr = ManuallyDrop::new(ManuallyDrop::new(()).into());
    PValue {
        kind: C,
        allocated: true,
        data: PValueData {ptr: },
    }
}
*/
fn main() {
    unsafe {
        register_a();
        register_b();
        //register_c();
    }
    
    println!("{}", PValue::invalid().to_string());
    
    println!("{}", PValue::int(15).to_string());
    
    println!("{}", make_a().to_string());
    
    println!("{}", make_b().to_string());
    
    //println!("{}", make_c(34).to_string());
}