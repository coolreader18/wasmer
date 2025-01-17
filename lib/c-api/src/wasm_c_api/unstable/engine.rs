//! Unstable non-standard Wasmer-specific types for the
//! `wasm_engine_t` and siblings.

use super::super::engine::{wasm_config_t, wasmer_compiler_t, wasmer_engine_t};
use super::target_lexicon::wasmer_target_t;

/// Unstable non-standard Wasmer-specific API to update the
/// configuration to specify a particular target for the engine.
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer_wasm.h"
/// #
/// int main() {
///     // Create the configuration.
///     wasm_config_t* config = wasm_config_new();
///
///     // Set the target.
///     {
///         wasmer_triple_t* triple = wasmer_triple_new_from_host();
///         wasmer_cpu_features_t* cpu_features = wasmer_cpu_features_new();
///         wasmer_target_t* target = wasmer_target_new(triple, cpu_features);
///
///         wasm_config_set_target(config, target);
///     }
///
///     // Create the engine.
///     wasm_engine_t* engine = wasm_engine_new_with_config(config);
///
///     // Check we have an engine!
///     assert(engine);
///
///     // Free everything.
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_set_target(config: &mut wasm_config_t, target: Box<wasmer_target_t>) {
    config.target = Some(target);
}

/// Check whether the given compiler is available, i.e. part of this
/// compiled library.
#[no_mangle]
pub extern "C" fn wasmer_is_compiler_available(compiler: wasmer_compiler_t) -> bool {
    match compiler {
        wasmer_compiler_t::CRANELIFT if cfg!(feature = "cranelift") => true,
        wasmer_compiler_t::LLVM if cfg!(feature = "llvm") => true,
        wasmer_compiler_t::SINGLEPASS if cfg!(feature = "singlepass") => true,
        _ => false,
    }
}

/// Check whether there is no compiler available in this compiled
/// library.
#[no_mangle]
pub extern "C" fn wasmer_is_headless() -> bool {
    !cfg!(feature = "compiler")
}

/// Check whether the given engine is available, i.e. part of this
/// compiled library.
#[no_mangle]
pub extern "C" fn wasmer_is_engine_available(engine: wasmer_engine_t) -> bool {
    match engine {
        wasmer_engine_t::JIT if cfg!(feature = "jit") => true,
        wasmer_engine_t::NATIVE if cfg!(feature = "native") => true,
        wasmer_engine_t::OBJECT_FILE if cfg!(feature = "object-file") => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use inline_c::assert_c;
    use std::env::{remove_var, set_var};

    #[test]
    fn test_wasmer_is_headless() {
        set_var(
            "COMPILER",
            if cfg!(feature = "compiler") { "0" } else { "1" },
        );

        (assert_c! {
            #include "tests/wasmer_wasm.h"
            #include <stdlib.h>

            int main() {
                assert(wasmer_is_headless() == (getenv("COMPILER")[0] == '1'));

                return 0;
            }
        })
        .success();

        remove_var("COMPILER");
    }

    #[test]
    fn test_wasmer_is_compiler_available() {
        set_var(
            "CRANELIFT",
            if cfg!(feature = "cranelift") {
                "1"
            } else {
                "0"
            },
        );
        set_var("LLVM", if cfg!(feature = "llvm") { "1" } else { "0" });
        set_var(
            "SINGLEPASS",
            if cfg!(feature = "singlepass") {
                "1"
            } else {
                "0"
            },
        );

        (assert_c! {
            #include "tests/wasmer_wasm.h"
            #include <stdlib.h>

            int main() {
                assert(wasmer_is_compiler_available(CRANELIFT) == (getenv("CRANELIFT")[0] == '1'));
                assert(wasmer_is_compiler_available(LLVM) == (getenv("LLVM")[0] == '1'));
                assert(wasmer_is_compiler_available(SINGLEPASS) == (getenv("SINGLEPASS")[0] == '1'));

                return 0;
            }
        })
        .success();

        remove_var("CRANELIFT");
        remove_var("LLVM");
        remove_var("SINGLEPASS");
    }

    #[test]
    fn test_wasmer_is_engine_available() {
        set_var("JIT", if cfg!(feature = "jit") { "1" } else { "0" });
        set_var("NATIVE", if cfg!(feature = "native") { "1" } else { "0" });
        set_var(
            "OBJECT_FILE",
            if cfg!(feature = "object-file") {
                "1"
            } else {
                "0"
            },
        );

        (assert_c! {
            #include "tests/wasmer_wasm.h"
            #include <stdlib.h>

            int main() {
                assert(wasmer_is_engine_available(JIT) == (getenv("JIT")[0] == '1'));
                assert(wasmer_is_engine_available(NATIVE) == (getenv("NATIVE")[0] == '1'));
                assert(wasmer_is_engine_available(OBJECT_FILE) == (getenv("OBJECT_FILE")[0] == '1'));

                return 0;
            }
        })
        .success();

        remove_var("JIT");
        remove_var("NATIVE");
        remove_var("OBJECT_FILE");
    }
}
