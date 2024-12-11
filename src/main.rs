use core::fmt;
use std::sync::{Arc, RwLock};

use node::*;
mod node;

fn main() {

    let mut nodes: Vec<Arc<RwLock<Node<i32>>>> = Vec::new();

    // It is important for all of the nodes to be in order for this function implemenetation.
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

    // This Deque stores both the values as well as the distance to get there (from the root node)
    let mut searched: Vec<Connection<i32>> = Vec::new();
    // This Deque stores the next values to be evaluated
    let mut to_be_searched: Vec<Connection<i32>> = Vec::new();

    // Adds the first value into the searched values
    searched.push(Connection::new(nodes[0].clone(), 0.0));

    // Goes through all the values in the array
    let idx = searched[0].get_value();
    for value in &nodes[idx as usize].clone().read().unwrap().connections {
        to_be_searched.push(value.clone());
    }

    while !to_be_searched.is_empty() {
        to_be_searched.sort_by(|a, b| b.cmp(a));

        // unwrap is fine here, the vec should never be empty (because of the while loop)
        let search_value = to_be_searched.pop().unwrap();

        // Adds the next values to the to_be_searched array
        for value in &nodes[search_value.get_value() as usize].clone().read().unwrap().connections {

            let mut new_value = value.clone();
            // Adds the previous distance to the value
            new_value.distance += search_value.distance;

            // Skips if the value already was searched
            if searched.contains(&new_value) {

                let pos = searched.iter().position(|x| x == &new_value).unwrap();

                if new_value.get_value() < searched[pos].get_value() {
                    searched.remove(pos);
                }

                else { continue; }

            }

            to_be_searched.push(new_value);
        }

        if searched.contains(&search_value) {

            let pos = searched.iter().position(|x| x == &search_value).unwrap();

            if search_value.get_value() < searched[pos].get_value() {
                searched.remove(pos);
            } else { continue; }

        }

        searched.push(search_value);


    }

    println!("Searched Values:");
    for value in &searched {
        println!(" -> {}", value);
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
