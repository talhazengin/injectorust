#![allow(dead_code)]

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::Debug;


/**** BASE KEY, SERVICE DEFINITIONS. ****/
pub trait Key: Debug + Clone + Ord + Any + Send + Sync 
{
}

pub trait Service: Any 
{
    type Key: Key;
    fn key() -> &'static Self::Key;
}

// impl<T> Key for T
//     where T: Debug + Clone + Ord + Any + Send + Sync 
// { 
// }


/**** CONTAINER. ****/

pub trait GenericRegistrar<TKey, TService>
    where TKey: Key, TService: Service
{
    fn register<T>(&mut self, svc: T) -> &mut Self
        where T: Service<Key = TKey> + Into<Box<TService>>;
}

// TODO: Consider why BTreeMap ? Why not HashMap or something?
pub struct Container<TKey, TService> {
    services: BTreeMap<TKey, TService>,
}

impl<TKey, TService> Container<TKey, TService> 
    where TKey: Key, TService: Any
{
    pub fn new() -> Self {
        Container { services: BTreeMap::new() }
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