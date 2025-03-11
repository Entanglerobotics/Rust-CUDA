// // Allow unsafe functions and types from the generated bindings.
// #![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// pub fn initialize_optix() -> Result<(), String> {
//     let result = unsafe { optixInit() };
//     if result == 0 {
//         Ok(())
//     } else {
//         Err(format!("Failed to initialize OptiX: Error code {}", result))
//     }
// }
