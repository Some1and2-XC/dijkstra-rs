use core::fmt;
use std::{fmt::Display, sync::{Arc, RwLock}};

#[derive(Clone)]
pub struct Node<T> {
    pub value: T,
    pub connections: Vec<Connection<T>>,
}

#[derive(Clone)]
pub struct Connection<T> {
    pub destination: Arc<RwLock<Node<T>>>,
    pub distance: f64,
}

impl<T: Display> Node<T> {

    /// Function for creating a new node.
    pub fn new(value: T) -> Self {
        return Self {
            value,
            connections: Vec::new(),
        };
    }

    /// Function for adding a connection to the node.
    pub fn add_connection(&mut self, connection: Connection<T>) -> () {
        self.connections.push(connection);
    }
}

impl<T: Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.value.cmp(&other.value));
    }
}

impl<T> Connection<T> {

    /// Function for creating a new node connection.
    pub fn new(destination: Arc<RwLock<Node<T>>>, distance: f64) -> Self {
        return Self {
            destination,
            distance,
        };
    }

}

impl<T: Copy> Connection<T> {

    /// Function for getting the reference value from a connection.
    pub fn get_value(&self) -> T {
        return self.destination.read().unwrap().value.clone();
    }

}

impl<T: Eq> PartialEq for Connection<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.destination.read().unwrap().value.eq(&other.destination.read().unwrap().value);
    }
}

impl<T: Ord> PartialOrd for Connection<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(&other));
    }
}

impl<T: Ord + Eq> Eq for Connection<T> { }

impl<T: Ord> Ord for Connection<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.distance.total_cmp(&other.distance);
    }
}

impl<T: Display> fmt::Display for Connection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Connection to {} with distance: {}", self.destination.read().unwrap().value, self.distance);
    }
}
