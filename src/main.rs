use deno_core::{error::AnyError, op, Extension};
use std::rc::Rc;
pub mod fft;

#[op]
fn op_my_cool_math_fft(
    real: Vec<f64>,
    imag: Vec<f64>,
    sample: usize,
) -> Result<(Vec<f64>, Vec<f64>), AnyError> {
    let result = fft::fft(real, imag, sample, false);
    Ok(result)
}

#[op]
fn op_my_cool_math_ifft(
    real: Vec<f64>,
    imag: Vec<f64>,
    sample: usize,
) -> Result<(Vec<f64>, Vec<f64>), AnyError> {
    let result = fft::fft(real, imag, sample, true);
    Ok(result)
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder("my_cool_deno")
        .ops(vec![
            op_my_cool_math_fft::decl(),
            op_my_cool_math_ifft::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    js_runtime
        .execute_script("deno-runtime:runtime.js", include_str!("./runtime.js"))
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js("./example.js")) {
        eprintln!("error: {}", error);
    }
}
