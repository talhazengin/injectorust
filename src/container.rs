#![allow(dead_code)]

use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

/**** BASE KEY, SERVICE DEFINITIONS ****/
pub trait Key: Debug + Clone + Eq + Any + Send + Sync + Hash
{
}

pub trait Service: Any 
{
    type Key: Key;
    fn key() -> &'static Self::Key;
}


/**** CONTAINER ****/


pub struct Container<TKey, TService> {
    pub services: HashMap<TKey, TService>,
}

impl<TKey, TService> Container<TKey, TService> 
    where TKey: Key, TService: Service
{
    pub fn new() -> Self {
        Container { services: HashMap::new() }
    }
}


/**** CONTAINER GENERIC FUNCTIONALTY ****/

pub trait GenericRegistrar<TKey, TService>
    where TKey: Key, TService: Service
{
    fn register<TTrait, TStruct>(&mut self) -> &mut Self
        where TStruct: Service<Key = TKey> + Into<Box<TService>>;
}

impl<TKey, TService> GenericRegistrar<TKey, TService> for Container<TKey, TService>
        where TKey: Key, TService: Service
{
    fn register<TTrait, TStruct>(&mut self) -> &mut Self
        where TStruct: Service<Key = TKey> + Into<Box<TService>>,
    {
        let service_type_id = std::any::TypeId::of::<TTrait>();
        let implementation_type_id = std::any::TypeId::of::<TStruct>();

        self.services.insert(service_type_id, implementation_type_id);

        self //.register(TStruct::key().clone())
    }
}