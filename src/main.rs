use rust_program_test::cap2::cap2::cap2;
use rust_program_test::cap3::cap3::cap3;
use rust_program_test::cap4::cap4::cap4;
use rust_program_test::cap5::cap5::cap5;
//use rust_program_test::cap5::cap5::cap5;


fn main() {
    #[cfg(feature = "cap2")]
    cap2();

    #[cfg(feature = "cap3")]
    cap3();

    #[cfg(feature = "cap4")]
    cap4();

    #[cfg(feature = "cap5")]
    cap5();

}
