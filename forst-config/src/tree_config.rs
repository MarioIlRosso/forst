pub struct TreeConfig {
    pub max_depth: Option<usize>,
    pub display_hidden: bool,
    pub display_sizes: bool,
    pub no_unicode: bool,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            max_depth: None,
            display_hidden: false,
            no_unicode: true,
            display_sizes: false,
        }
    }
}
