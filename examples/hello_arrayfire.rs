extern crate arrayfire as af;

fn main() {
    af::info().unwrap();

    let num_rows: u64 = 5;
    let num_cols: u64 = 5;
    let dims = af::Dim4::new(&[num_rows, num_cols, 1, 1]);

    println!("Multiple random {}-by-{} matrix of random floats with identity on the GPU",
             num_rows, num_cols);

    // generate and print
    let a = af::identity(dims, af::Aftype::F32);
    let b = af::randu(dims, af::Aftype::F32);

    // multiply
    // use closure in order to use `try!`
    // `move` in order to take ownership of `b`
    // now we can use `try!`, instead of nested `and_then`
    // basically I wanted to
    // ```
    // let c = {
    //   let a = try!(a);
    //   let b = try!(b);
    //   ....
    //   af::matmul(....);
    // }
    // ```
    // but we can't use `return` in block ("outside" fn)
    let c = a.and_then(move |a| {
        let b = try!(b);

        // print
        try!(af::print(&a));
        try!(af::print(&b));

        af::matmul(&a, &b, af::MatProp::NONE, af::MatProp::NONE)
    });

    // panic if error occurred
    c.and_then(|c| af::print(&c)).unwrap();
}
