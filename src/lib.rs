// ---------------------------------------------------------------------------------------------
// General helper traits

pub trait CollectJoin {
    fn join(&mut self, separator: &str) -> String
        where Self: Iterator,
              <Self as Iterator>::Item: ToString
    {
        self.map(|x| x.to_string()).collect::<Vec<_>>().join(separator)
    }

    fn to_vec(self) -> Vec<<Self as Iterator>::Item>
        where Self: Iterator + Sized
    {
        self.collect::<Vec<_>>()
    }
}

impl<I: Iterator> CollectJoin for I {}
