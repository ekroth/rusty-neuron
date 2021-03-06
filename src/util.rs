use std::iter;

// Flatten Iterator of Result into Result of Vec
pub fn result_sequence<T, U, E>(items: T) -> Result<Vec<U>, E>
    where T: iter::IntoIterator<Item=Result<U, E>>,
{
    let mut rxs = Ok(vec![]);

    for i in items {
        let mut xs = try!(rxs);
        let i = try!(i);
        xs.push(i);
        rxs = Ok(xs);
    }

    rxs
}

#[test]
fn sequence() {
    type VRS = Vec<Result<u32, i32>>;
    type RVS = Result<Vec<u32>, i32>;

    // empty vec should remain empty
    {
        let xrs: VRS = vec![];
        let rxs: RVS = Ok(vec![]);
        let orxs: RVS = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }

    // non-empty should be the same
    {
        let xrs: VRS = vec![Ok(1), Ok(2)];
        let rxs: RVS = Ok(vec![1, 2]);
        let orxs: RVS = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }

    // err should be err
    {
        let xrs: VRS = vec![Err(-1)];
        let rxs: RVS = Err(-1);
        let orxs: RVS = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }

    // err should propagate
    {
        let xrs: VRS = vec![Ok(1), Err(-1), Ok(2)];
        let rxs: RVS = Err(-1);
        let orxs: RVS = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }
}
