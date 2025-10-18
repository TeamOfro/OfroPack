mod close_issue;
mod comment;
mod generate_preview;
mod parse_issue;
mod post_extend_success;
mod post_failure;
mod post_success;
mod process_issue;
mod react;

use anyhow::Result;

use crate::cmd::{Run, Runner, RunnerSubcommands};

impl Run for Runner {
    fn run(&self) -> Result<()> {
        match &self.subcommand {
            RunnerSubcommands::ProcessIssue {
                issue_number,
                issue_type,
                body,
            } => process_issue::run(*issue_number, *issue_type, body),
            RunnerSubcommands::ParseIssue { body, issue_type } => {
                parse_issue::run(body, *issue_type)
            }
            RunnerSubcommands::PostSuccess {
                issue_number,
                pr_number,
                preview_url,
            } => post_success::run(*issue_number, *pr_number, preview_url),
            RunnerSubcommands::PostExtendSuccess {
                issue_number,
                pr_number,
                materials,
            } => post_extend_success::run(*issue_number, *pr_number, materials.clone()),
            RunnerSubcommands::PostFailure {
                issue_number,
                error_message,
                workflow_url,
            } => post_failure::run(*issue_number, error_message, workflow_url),
            RunnerSubcommands::Comment { issue_number, body } => comment::run(*issue_number, body),
            RunnerSubcommands::React {
                issue_number,
                reaction,
            } => react::run(*issue_number, reaction),
            RunnerSubcommands::CloseIssue { issue_number } => close_issue::run(*issue_number),
            RunnerSubcommands::GeneratePreview {
                source,
                model_name,
                preview_dir,
                repo_owner,
                repo_name,
                branch,
            } => generate_preview::run(
                source,
                model_name,
                preview_dir,
                repo_owner,
                repo_name,
                branch,
            ),
        }
    }
}
