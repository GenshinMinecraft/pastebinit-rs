trait ProviderTrait {
    fn upload_paste(content: String, title: Option<String>, public: bool) -> Result<String, String>;
}