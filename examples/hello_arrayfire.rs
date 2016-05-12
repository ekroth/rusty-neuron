extern crate arrayfire as af;

fn main() {
    af::info().unwrap();

    let num_rows: u64 = 5;
    let num_cols: u64 = 5;
    let dims = af::Dim4::new(&[num_rows, num_cols, 1, 1]);

    println!("Multiple random {}-by-{} matrix of random floats with identity on the GPU",
             num_rows, num_cols);

    let a = af::identity(dims, af::Aftype::F32);
    let b = af::randu(dims, af::Aftype::F32);
    let c = a.and_then(|a| {
        try!(af::print(&a));
        b.and_then(|b| {
            try!(af::print(&b));
            af::matmul(&a, &b, af::MatProp::NONE, af::MatProp::NONE)
        })
    }).unwrap();

    af::print(&c).unwrap();
}
