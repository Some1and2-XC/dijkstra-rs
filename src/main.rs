use core::fmt;
use std::sync::{RwLock, Arc};

use node::*;
mod node;

fn main() {

    let mut nodes: Vec<Arc<RwLock<Node<i32>>>> = Vec::new();

    nodes.push(Arc::new(RwLock::new(Node::new(0))));
    nodes.push(Arc::new(RwLock::new(Node::new(1))));
    nodes.push(Arc::new(RwLock::new(Node::new(2))));
    nodes.push(Arc::new(RwLock::new(Node::new(3))));
    nodes.push(Arc::new(RwLock::new(Node::new(4))));
    nodes.push(Arc::new(RwLock::new(Node::new(5))));
    nodes.push(Arc::new(RwLock::new(Node::new(6))));

    // Top connections
    add_conection(&mut nodes, 0, 2, 6.0 );
    add_conection(&mut nodes, 2, 3, 8.0 );
    add_conection(&mut nodes, 3, 4, 10.0);
    add_conection(&mut nodes, 4, 6, 2.0 );

    // Bottom connections
    add_conection(&mut nodes, 0, 1, 2.0 );
    add_conection(&mut nodes, 1, 3, 5.0 );
    add_conection(&mut nodes, 3, 5, 15.0);
    add_conection(&mut nodes, 5, 6, 6.0 );

    println!("Node 6 Connections:");
    for i in nodes[6].read().unwrap().connections.clone() {
        println!(" -> {}", i);
    }

}

/// Utility method for adding a connection to a node from the vec.
pub fn add_conection<T: fmt::Display>(nodes: &mut Vec<Arc<RwLock<Node<T>>>>, idx_1: usize, idx_2: usize, distance: f64) -> Option<()> {
    // Adds the connection to idx_1
    {
        let tmp_ptr = nodes.get_mut(idx_1)?.clone();
        let mut v = tmp_ptr.write().ok()?;
        v.add_connection(Connection::new(nodes.get(idx_2)?.clone(), distance));
    }

    // Adds the connection to idx_2
    {
        let tmp_ptr = nodes.get_mut(idx_2)?.clone();
        let mut v = tmp_ptr.write().ok()?;
        v.add_connection(Connection::new(nodes.get(idx_1)?.clone(), distance));
    }
    return Some(());
}
