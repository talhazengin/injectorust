#![allow(dead_code)]

use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

/**** BASE KEY, SERVICE DEFINITIONS ****/
pub trait Key: Debug + Clone + Ord + Any + Send + Sync + Hash
{
}

pub trait Service: Any 
{
    type Key: Key;
    fn key() -> &'static Self::Key;
}

/**** CONTAINER ****/

pub trait GenericRegistrar<TKey, TService>
    where TKey: Key, TService: Service
{
    fn register<T>(&mut self, svc: T) -> &mut Self
        where T: Service<Key = TKey> + Into<Box<TService>>;
}

pub struct Container<TKey, TService> {
    pub services: HashMap<TKey, TService>,
}

impl<TKey, TService> Container<TKey, TService> 
    where TKey: Key, TService: Any
{
    pub fn new() -> Self {
        Container { services: HashMap::new() }
    }

    pub fn register(&mut self, key: TKey, svc: Box<TService>) -> &mut Self {
        self
    }
}

impl<TKey, TService> GenericRegistrar<TKey, TService> for Container<TKey, TService>
        where TKey: Key, TService: Service
{
    fn register<T>(&mut self, svc: T) -> &mut Self
        where T: Service<Key = TKey> + Into<Box<TService>>,
    {
        self.register(T::key().clone(), svc.into())
    }
}