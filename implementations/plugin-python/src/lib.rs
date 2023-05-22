#[macro_use]
extern crate lazy_static;

pub mod wrap;
pub use wrap::*;

pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;

impl ModuleTrait for Module {
    fn generate_bindings(args: ArgsGenerateBindings) -> Result<Output, String> {
        let version = args.wrap_info.version;

        // First, ensure version is "0.1"
        if version != "0.1" {
            return Err(
                format!("Unsupported ABI Version - {}; Supported - 0.1", version)
            );
        }

        let wrap_info = args.wrap_info;
        let renderer = Renderer::new();
        let mut output = Output::new();

        output.files.push(File {
            name: "__init__.py".to_string(),
            data: renderer.render(
                "__init__.py",
                &None
            )
        });

        output.files.push(File {
            name: "module.py".to_string(),
            data: renderer.render(
                "module.py",
                &wrap_info.abi
            )
        });

        output.files.push(File {
            name: "types.py".to_string(),
            data: renderer.render(
                "types.py",
                &wrap_info.abi
            )
        });

        output.files.push(File {
            name: "wrap_info.py".to_string(),
            data: renderer.render(
                "wrap_info.py",
                &wrap_info
            )
        });

        Ok(output)
    }
}