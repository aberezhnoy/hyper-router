use std::collections::HashMap;
use std::hash::{Hash, BuildHasher};
use std::any::Any;
use std::borrow::Borrow;
use std::fmt::Debug;

pub trait InsertBoxed<K, V, S> {
    //fn get_downcast_ref<Q: ?Sized>(&self, k: &Q) -> Option<&V>;
    fn get_downcast_ref(&self, k: &K) -> Option<&V>;
}

impl<K, V, S> InsertBoxed<K, V, S> for HashMap<K, V, S>
    where K: Eq + Hash,
          V: Any + Debug,
          S: BuildHasher
{
    fn get_downcast_ref(&self, k: &K) -> Option<&V>
    where V: Any + Debug

    {
        let x = self.get(k);

        if x.is_none() {
            return None;
        }



        let y: &Any = x.unwrap();

        println!("{:?}", y.downcast_ref::<String>());
        //println!("{:?}", y.get_type_id());

        //let y: Box<Any> = x.unwrap();
        //let y: Any = x.unwrap();

        //return y.downcast_ref::<T>();
        return x;
    }
}

pub fn get_downcast_ref<T>(key: &str, map: HashMap<&str, Box<Any>>) -> Option<&'static T> where T: Debug + 'static {
    let x = map.get(key).unwrap();
    let r = x.downcast_ref::<T>();

    println!("{:?}", r.unwrap());

    return Some(r.unwrap());
}

//+ 'static