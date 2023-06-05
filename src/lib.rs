use actionidmap_updater::ActionIDMap;

mod tests;
mod actions;

#[derive(Debug)]
pub struct Agent {
    pub actionmap: ActionIDMap<String, String>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            actionmap: ActionIDMap::new(
                "https://raw.githubusercontent.com/sigaloid/dolos/main/data/GqlOperations.json"
                    .into(),
                86400,
            )
            .unwrap(),
        }
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}
