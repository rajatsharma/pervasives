pub mod file;
pub mod list;
pub mod proc;

#[cfg(test)]
mod tests {
    use super::*;
    use list::List;

    #[test]
    fn it_works() {
        let list = list![1, 2, 3]
            .filter(|x| *x == 1)
            .map(|x| x * 2)
            .fold(0, |x, y| x + y);
    }
}
