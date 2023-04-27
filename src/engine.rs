use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::Extension;
use std::rc::Rc;


pub async fn girlfriend(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let girlfriend_extension = Extension::builder("girlfriend")
        .esm(include_js_files!(
            "./polyfill/core.js",
        ))
        .ops(vec![
            crate::file::read::op_read_file::decl(),
            crate::file::write::op_write_file::decl(),
            crate::file::remove::op_remove::decl(),
            crate::file::fetch::op_fetch::decl(),
            crate::shell::mkdir::op_make_directory::decl(),
            crate::shell::cd::op_current_directory::decl(),
            crate::shell::ls::op_list_directory::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(crate::typescript::TsModuleLoader)),
        extensions: vec![girlfriend_extension],
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}