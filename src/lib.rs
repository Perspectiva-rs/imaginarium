extern crate compositio;

use std::ops::Index;
use compositio::OwnedMatrix;

// type Image<T> = compositio::Matrix<T,Vec<T>,Vec<usize>>;
struct Image<T>(OwnedMatrix<T>);

impl<'a,T> Index<&'a [usize]> for Image<T>{
    type Output = T;    
    fn index(& self, index:&[usize]) -> &T{
        &self.0[index]
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
