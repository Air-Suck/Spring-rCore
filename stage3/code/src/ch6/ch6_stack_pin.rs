use std::pin::Pin;
use std::marker::PhantomPinned;

#[allow(dead_code)]
pub fn stack_pin() {
    // test1 is safe to move before we initialize it
    let mut test1 = Test::new("test1");
    // Notice how we shadow `test1` to prevent it from being accessed again
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    std::mem::swap(&mut test1, &mut test2);                  //change Pin instead of struct

    // std::mem::swap(test1.get_mut(), test2.get_mut());        //ERROR!

    println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
}

#[allow(dead_code)]
pub fn limof_stack_pin() {
    let mut test1 = Test::new("test1");
    let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1_pin.as_mut());
    drop(test1_pin);    // simulate to exit the func frame
    
    let mut test2 = Test::new("test2");
    std:: mem::swap(&mut test1, &mut test2);
    println!("Not self referential anymore: {:?}", test1.b);
}

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Unpin for Test {
    
}

#[allow(dead_code)]
impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }
    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}