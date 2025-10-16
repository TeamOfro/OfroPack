use std::path::Path;

use clap::Parser;

mod cli;
mod constants;
mod file_utils;
mod gallery;
mod image_validator;
mod models;
mod processor;
mod runner;

use cli::{Cli, Commands, RunnerCommands};
use gallery::GalleryGenerator;
use processor::Processor;
use runner::{GitHubClient, IssueParser, IssueProcessor, PreviewGenerator};

fn main() {
    if let Err(err) = run() {
        eprintln!("\n❌ エラー:\n{}", err);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Validate that we're in a resource pack directory (except for generate-gallery and runner commands)
    match &cli.command {
        Commands::GenerateGallery { .. } | Commands::Runner { .. } => {}
        _ => {
            if !Path::new("assets").exists() {
                anyhow::bail!(
                    "assetsディレクトリが存在しません。\n\
                    Minecraftリソースパックのルートディレクトリで実行してください。"
                );
            }
        }
    }

    cli.validate()?;

    match &cli.command {
        Commands::Add {
            materials,
            custom_model_data,
            path_to_image,
        } => {
            let custom_model_data = cli::get_custom_model_data(custom_model_data, path_to_image)?;
            let normalized_materials = cli::normalize_materials(materials);

            let processor = Processor::new(custom_model_data);
            processor.add_with_texture(&normalized_materials, path_to_image)?;
        }
        Commands::Extend {
            materials,
            custom_model_data,
        } => {
            let custom_model_data = custom_model_data.to_lowercase();
            let normalized_materials = cli::normalize_materials(materials);

            let processor = Processor::new(custom_model_data);
            processor.extend_materials(&normalized_materials)?;
        }
        Commands::GenerateGallery { output } => {
            let generator = GalleryGenerator::new(output.clone());
            generator.generate()?;
        }
        Commands::Runner { subcommand } => match subcommand {
            RunnerCommands::ProcessIssue { issue_number, body } => {
                let processor = IssueProcessor::new()?;
                let result = processor.process(*issue_number, body)?;

                // Output for GitHub Actions (preview_url, custom_model_data)
                println!("PREVIEW_URL={}", result.preview_url);
                println!("CUSTOM_MODEL_DATA={}", result.custom_model_data);
            }
            RunnerCommands::PostSuccess {
                issue_number,
                pr_number,
                preview_url,
            } => {
                let processor = IssueProcessor::new()?;
                processor.post_success(*issue_number, *pr_number, preview_url)?;
            }
            RunnerCommands::PostFailure {
                issue_number,
                error_message,
                workflow_url,
            } => {
                let processor = IssueProcessor::new()?;
                processor.post_failure(*issue_number, error_message, workflow_url)?;
            }
            RunnerCommands::ParseIssue { body } => {
                let parsed = IssueParser::parse(body)?;
                println!("{}", IssueParser::output_github_actions(&parsed));
            }
            RunnerCommands::Comment { issue_number, body } => {
                let client = GitHubClient::from_env()?;
                client.comment_issue(*issue_number, body)?;
            }
            RunnerCommands::React {
                issue_number,
                reaction,
            } => {
                let client = GitHubClient::from_env()?;
                client.react_issue(*issue_number, reaction)?;
            }
            RunnerCommands::CloseIssue { issue_number } => {
                let client = GitHubClient::from_env()?;
                client.close_issue(*issue_number)?;
            }
            RunnerCommands::GeneratePreview {
                source,
                model_name,
                preview_dir,
                repo_owner,
                repo_name,
                branch,
            } => {
                let generator = PreviewGenerator::new(preview_dir);
                let preview_path = generator.generate(source, model_name)?;

                // Generate URL if repo info provided
                if let (Some(owner), Some(name), Some(br)) = (repo_owner, repo_name, branch) {
                    let url = PreviewGenerator::generate_url(owner, name, br, &preview_path);
                    println!("preview_url={}", url);
                }
            }
        },
    }

    Ok(())
}
