// Flatten Iterator of Result into Result of Vec
pub fn result_sequence<T, U, E>(items: T) -> Result<Vec<U>, E>
    where T: IntoIterator<Item=Result<U, E>>
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
    // empty vec should remain empty
    {
        let xrs: Vec<Result<u32, i32>> = vec![];
        let rxs = Ok(vec![]);
        let orxs = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }

    // non-empty should be the same
    {
        let xrs: Vec<Result<u32, i32>> = vec![Ok(1), Ok(2)];
        let rxs = Ok(vec![1, 2]);
        let orxs = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }

    // err should be err
    {
        let xrs: Vec<Result<u32, i32>> = vec![Err(-1)];
        let rxs = Err(-1);
        let orxs = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }

    // err should propagate
    {
        let xrs: Vec<Result<u32, i32>> = vec![Ok(1), Err(-1), Ok(2)];
        let rxs = Err(-1);
        let orxs = result_sequence(xrs);
        assert_eq!(rxs, orxs);
    }
}
