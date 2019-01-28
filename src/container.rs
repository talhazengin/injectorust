#![allow(dead_code)]

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::Debug;


/**** BASE KEY, SERVICE DEFINITIONS. ****/
pub trait Key: Debug + Clone + Ord + Any + Send + Sync 
{
}

impl<T> Key for T
    where T: Debug + Clone + Ord + Any + Send + Sync 
{ 
}

pub trait Service: Any + Sized 
{
    type Key: Key;
    fn key() -> &'static Self::Key;
}


/**** CONTAINER. ****/

// TODO: Consider why BTreeMap ? Why not HashMap or something?
pub struct Container<TKey: Sized, TService: Sized> {
    services: BTreeMap<TKey, TService>,
}

impl<TKey: Sized, TService: Sized> Container<TKey, TService> 
    where TKey: Key, TService: Any
{
    pub fn new() -> Self {
        Container { services: BTreeMap::new() }
    }

    pub fn register_service(&mut self, key: TKey, svc: Box<TService>) -> &mut Self {
        self
    }

    pub fn register<T>(&mut self, svc: T) -> &mut Self
        where T: Service<Key = TKey> + Into<Box<TService>>,
    {
        self.register_service(T::key().clone(), svc.into())
    }
}