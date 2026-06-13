use cargo_generate::{generate, GenerateArgs, TemplatePath};

// TODO: replace with the pushed template repo URL
const TEMPLATE_REPO: &str = "https://github.com/Vanuan/gpui-starter-template";

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("new") => {
            let name = args.next();

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
            eprintln!("Usage: gpui-cli new <name>");
        }
    }

    Ok(())
}
