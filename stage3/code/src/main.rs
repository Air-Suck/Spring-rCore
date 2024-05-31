//这里相当于是声明了要编译的块
mod ch8;
//绿色线程的代码跑不了，先注释掉
// mod green_thread;
mod ch4;
mod ch5;

//引入相关函数
#[allow(unused_imports)]
use crate::ch8::future;
#[allow(unused_imports)]
// use crate::green_thread::gthread;
#[allow(unused_imports)]
use crate::ch4::trait_imp;
#[allow(unused_imports)]
use crate::ch4::fat_ptr;
#[allow(unused_imports)]
use crate::ch5::generator_exp2;
#[allow(unused_imports)]
use crate::ch5::generator_exp3;

fn main() {
    // trait_imp();
    // fat_ptr();
    // gthread();
    generator_exp2();
    // generator_exp3();
    // future();
}