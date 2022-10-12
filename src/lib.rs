pub struct List<T>(Vec<T>);

impl<T> List<T> {
    fn map(&self, fmap: Box<dyn Fn(&T) -> T>) -> List<T> {
        List(self.0.iter().map(fmap).collect::<Vec<T>>())
    }

    fn filter(self, predicate: Box<dyn Fn(&T) -> bool>) -> List<T> {
        List(self.0.into_iter().filter(predicate).collect::<Vec<T>>())
    }

    fn vec(self) -> Vec<T> {
        self.0
    }
}

macro_rules! f {
    ($e:expr) => (Box::new($e))
}

macro_rules! list {
    ($($x:expr,)*) => (vec![$($x),*])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = list![1, 2, 3].filter(f!(|x| x == 1)).map(f!(|x| x * 2));
    }
}
