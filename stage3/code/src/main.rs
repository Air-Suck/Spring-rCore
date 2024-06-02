//使用!Unpin的特性标志
#![feature(negative_impls)]
//这里相当于是声明了要编译的块
//绿色线程的代码跑不了，先注释掉
// mod green_thread;
mod ch4;
mod ch5;
mod ch6;
mod ch8;

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
#[allow(unused_imports)]
use crate::ch6::{self_ref,self_ref_con,stack_pin,limof_stack_pin,heap_pin,pinned_gen};

fn main() {
    // gthread();           //绿色线程
    // trait_imp();         //trait，或者说唤醒器的实现
    // fat_ptr();           //胖指针
    // generator_exp2();    //无swap自指生成器
    // generator_exp3();    //有swap自指生成器
    // self_ref();          //自指结构体
    // self_ref_con();      //自指结构体，convince版本
    // stack_pin();         //Stack Pin
    // limof_stack_pin();   //Stack Frame Limit of Stack Pin
    // heap_pin();          //Heap Pin
    future();
}