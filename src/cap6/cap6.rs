use crate::cap6::define_enum::define_enum;
use crate::cap6::example_enum::option;
use crate::cap6::example_enum::impl_meun;
use crate::cap6::example_match::example_match;
use crate::cap6::example_if_let::example_if_let;

pub fn cap6() {
    define_enum();
    option();
    impl_meun();
    example_match();
    example_if_let(None);
    example_if_let(Some(5));
}