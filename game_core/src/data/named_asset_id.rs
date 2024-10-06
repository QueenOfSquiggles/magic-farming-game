use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct NamedAssets<T: Asset> {
    cache: HashMap<String, Handle<T>>,
}

impl<T: Asset> NamedAssets<T> {
    pub fn new() -> Self {
        Self {
            cache: Default::default(),
        }
    }

    pub fn register(&mut self, name: impl Into<String>, handle: Handle<T>) -> Option<Handle<T>> {
        self.cache.insert(name.into(), handle)
    }
    pub fn get(&self, name: impl Into<String>) -> Option<Handle<T>> {
        self.cache.get(&name.into()).cloned()
    }
}
