pub trait SampleStream {
    type Sample;

    fn next_sample(&mut self) -> Self::Sample;
}

macro_rules! iterator {
    ($type:ty) => {
        impl Iterator for $type {
            type Item = <$type as SampleStream>::Sample;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.next_sample())
            }
        }
    };
}
