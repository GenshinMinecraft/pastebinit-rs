pub type PasteBinUrl = String;
pub type ErrorMessage = String;

pub trait ProviderTrait {
    fn upload_paste(
        content: String,
        title: String,
        private: bool,
        raw: bool,
    ) -> Result<PasteBinUrl, ErrorMessage>;
}
