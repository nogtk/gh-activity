use super::GhOption;
use inquire::{list_option::ListOption, validator::Validation, Confirm, MultiSelect};

pub fn build() -> GhOption {
    let c = Confirm::new("Do you use json format?")
        .with_default(false)
        .prompt();

    match c {
        Ok(true) => {
            let validator = |input: &[ListOption<&&str>]| {
                let selected = input.len() == 0;
                match selected {
                    true => Ok(Validation::Invalid(
                        "You must choose at least one field".into(),
                    )),
                    false => Ok(Validation::Valid),
                }
            };

            let chose_fields = MultiSelect::new("Choose fields", fields())
                .with_validator(validator)
                .with_help_message("check/uncheck by space key")
                .prompt()
                .unwrap()
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(",");

            GhOption {
                arg: Some(String::from("--json")),
                content: Some(chose_fields),
            }
        }
        Ok(false) => GhOption {
            arg: None,
            content: None,
        },
        Err(_) => panic!("occur some errors"),
    }
}

fn fields() -> Vec<&'static str> {
    vec![
        "additions",
        "assignees",
        "author",
        "autoMergeRequest",
        "baseRefName",
        "body",
        "changedFiles",
        "closed",
        "closedAt",
        "comments",
        "commits",
        "createdAt",
        "deletions",
        "files",
        "headRefName",
        "headRefOid",
        "headRepository",
        "headRepositoryOwner",
        "id",
        "isCrossRepository",
        "isDraft",
        "labels",
        "latestReviews",
        "maintainerCanModify",
        "mergeCommit",
        "mergeStateStatus",
        "mergeable",
        "mergedAt",
        "mergedBy",
        "milestone",
        "number",
        "potentialMergeCommit",
        "projectCards",
        "projectItems",
        "reactionGroups",
        "reviewDecision",
        "reviewRequests",
        "reviews",
        "state",
        "statusCheckRollup",
        "title",
        "updatedAt",
        "url",
    ]
}
