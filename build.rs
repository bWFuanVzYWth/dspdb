use build_utils::{item::gen_items, recipe::gen_recipes};

mod build_utils;

fn main() {
    gen_items();
    gen_recipes();
}
