//在mod文件中先引入mod
mod ch6_pin;
mod ch6_self_ref;

pub use ch6_pin::pin;
//使用pub关键字使self_ref, self_ref_con对其他的crate可见
pub use ch6_self_ref::{self_ref, self_ref_con};