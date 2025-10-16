use anyhow::Result;

use crate::runner::GitHubClient;

pub fn run(issue_number: u64, reaction: &str) -> Result<()> {
    // Validate reaction
    let valid_reactions = [
        "+1", "-1", "laugh", "confused", "heart", "hooray", "rocket", "eyes",
    ];
    if !valid_reactions.contains(&reaction) {
        anyhow::bail!(
            "無効なReactionです。有効なReaction: {}",
            valid_reactions.join(", ")
        );
    }

    let client = GitHubClient::from_env()?;
    client.react_issue(issue_number, reaction)?;
    Ok(())
}
