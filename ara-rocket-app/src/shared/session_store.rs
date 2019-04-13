use std::collections::HashMap;
use std::hash::Hash;

pub trait SessionStore<K, V> {
    fn is_valid(&self, key: &K) -> bool;
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
}

pub struct HashMapSessionStore<K, V>
where
    K: Eq + Hash,
{
    store: HashMap<K, V>,
}

impl<K, V> HashMapSessionStore<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}
impl<K, V> SessionStore<K, V> for HashMapSessionStore<K, V>
where
    K: Eq + Hash,
{
    fn is_valid(&self, key: &K) -> bool {
        self.store.contains_key(key)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.store.get_mut(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.store.insert(key, value)
    }
}

/*
impl<'a, 'r> FromRequest<'a, 'r> for &'a mut HashMapSessionStore<String,User> {

    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        use std::borrow::BorrowMut;
        use std::ops::DerefMut;

        let store_state = request.guard::<State<'_, Arc<Mutex<HashMapSessionStore<String,User>>>>>()?;
        let mut store = store_state.inner().clone().lock().map(|mut st|Outcome::Success(st.deref_mut())).unwrap_or_else(|e|Outcome::Failure((Status::ServiceUnavailable, ())));
        store
    }
}
*/
