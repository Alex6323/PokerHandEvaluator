pub mod big_luts;
pub mod constants;
pub mod eval;
pub mod models;

pub fn test() {
    println!("It works");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
