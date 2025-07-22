mod guessing_game;
mod variable_test;
mod function_test;
mod control_flow_test;
mod ownership_test;
mod ToDoItem;
mod iterator_test;
mod structure_test;
mod enum_test;
mod collection_test;

use std::collections::{hash_map, HashMap};
use std::iter::Map;
use std::ptr::hash;

fn main() {

    // guessing_game::play_game();
    // variable_test::variable_test();
    // function_test::function_test()
    // control_flow_test::control_flow_test();
    // ownership_test::ownership_test();
    // ToDoItem::main();
    // ToDoItem::output();
    // iterator_test::main();
    // structure_test::main();
    // enum_test::main();
    // let hash_map: HashMap<String, String> = HashMap::<String,String>::new();
    collection_test::main();

}