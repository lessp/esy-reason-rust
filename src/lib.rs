#[macro_use]
extern crate ocaml;
use ocaml::{ToValue};

caml!(ml_send_float(f){
    return (f.f64_val() * 1.0).to_value();
});
