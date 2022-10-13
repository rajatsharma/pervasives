pub struct List<T>(Vec<T>);

impl<T> List<T> {
    pub fn map<U>(&self, fmap: U) -> List<T>
    where
        U: FnMut(&T) -> T,
    {
        List(self.0.iter().map(fmap).collect::<Vec<T>>())
    }

    pub fn filter<U>(self, predicate: U) -> List<T>
    where
        U: FnMut(&T) -> bool,
    {
        List(self.0.into_iter().filter(predicate).collect::<Vec<T>>())
    }

    pub fn fold<U, V>(&self, init: U, folder: V) -> U
    where
        V: FnMut(U, &T) -> U,
    {
        self.0.iter().fold(init, folder)
    }

    pub fn vec(self) -> Vec<T> {
        self.0
    }
}

#[macro_export]
macro_rules! list {
    () => (::list::List(vec![]));
    ($($x:literal),+ $(,)?) => (List(vec![$($x),+]));
}
