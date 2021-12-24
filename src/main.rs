use rust_program_test::cap2::cap2::cap2;
use rust_program_test::cap3::cap3::cap3;
use rust_program_test::cap4::cap4::cap4;
use rust_program_test::cap5::cap5::cap5;
use rust_program_test::cap6::cap6::cap6;
use rust_program_test::cap7::cap7::cap7;
use rust_program_test::cap7::mod_path as pub_as;
use rust_program_test::cap8::cap8::cap8;

fn main() {
    #[cfg(feature = "cap2")]
    cap2();

    #[cfg(feature = "cap3")]
    cap3();

    #[cfg(feature = "cap4")]
    cap4();

    #[cfg(feature = "cap5")]
    cap5();

    #[cfg(feature = "cap6")]
    cap6();

    #[cfg(feature = "cap7")]
    cap7();
    #[cfg(feature = "cap7")]
    pub_as::re_export();

    #[cfg(feature = "cap8")]
    cap8();
}
