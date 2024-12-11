use std::sync::{RwLock, Arc};

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

impl<T> Node<T> {

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

impl<T> Connection<T> {

    /// Function for creating a new node connection.
    pub fn new(destination: Arc<RwLock<Node<T>>>, distance: f64) -> Self {
        return Self {
            destination,
            distance,
        };
    }
}
