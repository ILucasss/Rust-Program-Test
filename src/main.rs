use rust_program_test::cap2::number_guess::number_guess;
use rust_program_test::cap3::var_and_mut::var_and_mut;
use rust_program_test::cap3::shadowing::shadowing;

fn main() {
    #[cfg(feature = "number_guess")]
    number_guess();

    #[cfg(feature = "Var_and_Mut")]
    var_and_mut();

    #[cfg(feature = "shadowing")]
    shadowing();

}
