use std::rc::Rc;

pub trait SampleStream {
    fn next_sample(&self) -> f32;
}

impl<T> SampleStream for Rc<T>
where
    T: SampleStream,
{
    fn next_sample(&self) -> f32 {
        (**self).next_sample()
    }
}

macro_rules! iterator {
    ($type:ty) => {
        impl Iterator for $type {
            type Item = f32;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.next_sample())
            }
        }
    };
}
