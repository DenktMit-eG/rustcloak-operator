use kube::{Resource, ResourceExt, runtime::reflector::ObjectRef};
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

#[derive(Debug)]
pub struct RefWatcher<C, W>
where
    C: Resource,
    C::DynamicType: Default + std::fmt::Debug,
    W: Resource,
    W::DynamicType: Default + std::fmt::Debug,
{
    phantom: std::marker::PhantomData<(C, W)>,
    refs: RwLock<HashMap<ObjectRef<W>, HashSet<String>>>,
}

impl<C, W> Default for RefWatcher<C, W>
where
    C: Resource,
    C::DynamicType: Default + std::fmt::Debug,
    W: Resource,
    W::DynamicType: Default + std::fmt::Debug,
{
    fn default() -> Self {
        Self {
            phantom: std::marker::PhantomData,
            refs: RwLock::new(HashMap::new()),
        }
    }
}

impl<C, W> RefWatcher<C, W>
where
    C: Resource,
    C::DynamicType: Default + std::fmt::Debug + Eq + std::hash::Hash + Clone,
    W: Resource,
    W::DynamicType: Default + std::fmt::Debug + Eq + std::hash::Hash + Clone,
{
    pub fn watch(&self, obj: &W) -> Vec<ObjectRef<C>> {
        let refs = self.refs.read().unwrap();
        let obj = ObjectRef::from(obj);
        refs.get(&obj)
            .into_iter()
            .flatten()
            .map(|v| ObjectRef::new(v))
            .map(|v| {
                if let Some(ns) = &obj.namespace {
                    v.within(ns)
                } else {
                    v
                }
            })
            .collect()
    }

    pub fn add<I, S>(&self, obj: &C, watched: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let obj = ObjectRef::from(obj);
        let mut refs = self.refs.write().unwrap();
        for w in watched {
            let obj = obj.clone();
            let mut key = ObjectRef::new(w.as_ref());
            if let Some(ns) = &obj.namespace {
                key = key.within(ns);
            }
            if let Some(v) = refs.get_mut(&key) {
                v.insert(obj.name);
            } else {
                refs.insert(key, [obj.name].into());
            }
        }
    }

    pub fn remove(&self, obj: &C) {
        let name = obj.name_unchecked();
        let mut refs = self.refs.write().unwrap();
        for (_, v) in refs.iter_mut() {
            v.retain(|x| x != &name);
        }
        refs.retain(|_, v| !v.is_empty());
    }
}
