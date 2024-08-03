#![allow(unused_imports)]

macro_rules! pub_mod_use {
    ($($name:ident),+) => {
        $(
        mod $name;
        pub use $name::*;
        )+
    };
}

pub_mod_use! {
    token,
    response,
    status,
    announcement,
    ge_item,
    event,
    resource,
    skill,
    drop_rate,
    monster,
    item,
    single_item,
    item_effect,
    craft,
    simple_item
}
