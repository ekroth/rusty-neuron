extern crate arrayfire as af;

pub struct Network;

use self::af::{Array, AfError};

impl Network {
    // create biases from an Iterator of sizes
    fn create_biases<T>(sizes: T) -> Result<Vec<Array>, AfError>
        where T: Iterator<Item=u64>
    {
        let dims = |y| af::Dim4::new(&[y, 1, 1, 1]);
        let mut rxs = Ok(vec![]);

        for y in sizes {
            let mut xs = try!(rxs);
            let arr = try!(af::randu(dims(y), af::Aftype::F32));
            xs.push(arr);
            rxs = Ok(xs);
        }

        rxs
    }
}
