use core::fmt;
use std::{fmt::Display, sync::{Arc, RwLock}};

#[derive(Clone)]
pub struct Node<T: Display> {
    pub value: T,
    pub connections: Vec<Connection<T>>,
}

#[derive(Clone)]
pub struct Connection<T: Display> {
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

    pub fn add_connection(&mut self, connection: Connection<T>) -> () {
        self.connections.push(connection);
    }
}

impl<T: Display> Connection<T> {

    /// Function for creating a new node connection.
    pub fn new(destination: Arc<RwLock<Node<T>>>, distance: f64) -> Self {
        return Self {
            destination,
            distance,
        };
    }

}

impl<T: Display> fmt::Display for Connection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Connection to {} with distance: {}", self.destination.read().unwrap().value, self.distance);
    }
}
