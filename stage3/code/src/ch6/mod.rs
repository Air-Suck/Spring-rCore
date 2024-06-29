//在mod文件中先引入mod
mod ch6_stack_pin;
mod ch6_self_ref;
mod ch6_heap_pin;
mod ch6_gen_and_pin;

//使用pub关键字使self_ref, self_ref_con对其他的crate可见
pub use ch6_stack_pin::{stack_pin,limof_stack_pin};
pub use ch6_self_ref::{self_ref, self_ref_con};
pub use ch6_heap_pin::heap_pin;
pub use ch6_gen_and_pin::pinned_gen;