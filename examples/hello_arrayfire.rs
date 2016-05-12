extern crate arrayfire as af;

#[allow(unused_must_use)]
fn main() {
    let num_rows: u64 = 5;
    let num_cols: u64 = 3;
    let dims = af::Dim4::new(&[num_rows, num_cols, 1, 1]);
    println!("Create a 5-by-3 matrix of random floats on the GPU");
    let a = match af::randu(dims, af::Aftype::F32) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };
    af::print(&a);
}
