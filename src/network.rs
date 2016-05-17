extern crate arrayfire as af;

pub struct Network;

use self::af::{Array, AfError};

impl Network {
    // create biases from an Iterator of sizes
    fn create_biases<T>(sizes: T) -> Result<Vec<Array>, AfError>
        where T: IntoIterator<Item=u64>
    {
        let dims = |y| af::Dim4::new(&[y, 1, 1, 1]);
        let rxs = sizes.into_iter().map(|y| af::randu(dims(y), af::Aftype::F32));
        ::util::result_sequence(rxs)
    }
}
