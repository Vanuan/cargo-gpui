use cargo_generate::{generate, GenerateArgs, TemplatePath};

const TEMPLATE_REPO: &str = "https://github.com/Vanuan/gpui-starter-template";

fn main() -> anyhow::Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // `cargo gpui new my-app` invokes `cargo-gpui` with args ["gpui", "new", "my-app"].
    // Strip the leading "gpui" so the same binary also works when run directly
    // (e.g. `cargo-gpui new my-app`).
    if args.first().map(String::as_str) == Some("gpui") {
        args.remove(0);
    }

    match args.first().map(String::as_str) {
        Some("new") => {
            let name = args.get(1).cloned();

            let generate_args = GenerateArgs {
                template_path: TemplatePath {
                    git: Some(TEMPLATE_REPO.to_string()),
                    ..TemplatePath::default()
                },
                name,
                ..GenerateArgs::default()
            };

            generate(generate_args)?;
        }
        _ => {
            eprintln!("Usage: cargo gpui new <name>");
        }
    }

    Ok(())
}
