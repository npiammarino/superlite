extern crate table;

use std::mem;
use table::{PAGE_SIZE, ROW_SIZE};

// node types
enum Node {
    Leaf,
    Internal,
}

use crate::Node::*;

// common
const NODE_TYPE_SIZE: usize = mem::size_of::<Node>();
const NODE_TYPE_OFFSET: usize = 0;
const IS_ROOT_SIZE: usize = mem::size_of::<u8>();
const IS_ROOT_OFFSET: usize = NODE_TYPE_SIZE;
const PARENT_POINTER_SIZE: usize = mem::size_of::<u32>();
const PARENT_POINTER_OFFSET: usize = IS_ROOT_OFFSET + IS_ROOT_SIZE;
const COMMON_NODE_HEADER_SIZE: usize = NODE_TYPE_SIZE + IS_ROOT_SIZE + PARENT_POINTER_SIZE;

// leaf header
const LEAF_NODE_NUM_CELLS_SIZE: usize = mem::size_of::<u32>();
const LEAF_NODE_NUM_CELLS_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
const LEAF_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + LEAF_NODE_NUM_CELLS_SIZE;

// leaf body
const LEAF_NODE_KEY_SIZE: usize = mem::size_of::<u32>();
const LEAF_NODE_KEY_OFFSET: usize = 0;
const LEAF_NODE_VALUE_SIZE: usize = ROW_SIZE;
const LEAF_NODE_VALUE_OFFSET: usize = LEAF_NODE_KEY_OFFSET + LEAF_NODE_KEY_SIZE;
const LEAF_NODE_CELL_SIZE: usize = LEAF_NODE_KEY_SIZE + LEAF_NODE_VALUE_SIZE;
const LEAF_NODE_SPACE_FOR_CELLS: usize = PAGE_SIZE - LEAF_NODE_HEADER_SIZE;
const LEAF_NODE_MAX_CELLS: usize = LEAF_NODE_SPACE_FOR_CELLS / LEAF_NODE_CELL_SIZE;

pub fn leaf_node_num_cells(node: usize) -> usize {
    node + LEAF_NODE_NUM_CELLS_OFFSET
}

pub fn leaf_node_cell(node: usize, cell_num: usize) -> usize {
    node + LEAF_NODE_HEADER_SIZE + cell_num * LEAF_NODE_CELL_SIZE
}

pub fn leaf_node_key(node: usize, cell_num: usize) -> usize {
    leaf_node_cell(node, cell_num)
}

pub fn leaf_node_value(node: usize, cell_num: usize) -> usize {
    leaf_node_cell(node, cell_num) + LEAF_NODE_KEY_SIZE
}

// pub fn initialize_leaf_node(node: usize) {
//     leaf_node_num_cells(node) = 0;
// }
