use async_trait::async_trait;
use std::any::Any;

/// Implementing struct supports fetching did documents by their id.
#[async_trait]
pub trait DidResolver {
    /// Checks given DID document.
    /// A DID document is considered as valid if returning ().
    /// Resolver may throw to indicate
    /// - that it is not responsible for this DID
    /// - that it considers this DID as invalid
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to check document for
    /// * `value` - value to check
    pub async fn check_did(&self, did_name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Gets document for given did name.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to fetch
    async fn get_did_document(&self, key: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// Sets document for given did name.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to set value for
    /// * `value` - value to set
    async fn set_did_document(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>>;
}

/// Implementing struct supports logging, for now only `log` is supported.
pub trait Logger {
    /// Cast to `Any` for downcasting,
    /// see https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object.
    fn as_any(&self) -> &dyn Any;

    /// Logs given message with given level.
    /// 
    /// # Arguments
    ///
    /// * `message` - message to log
    /// * `level` - optional arguments for logging level, levels may differ based on environment
    fn log(&self, message: &str, level: Option<&str>);
}

/// Implementing struct supports fetching vc documents by their id.
#[async_trait]
pub trait VcResolver {
    /// Checks given VC document.
    /// A VC document is considered as valid if returning ().
    /// Resolver may throw to indicate
    /// - that it is not responsible for this VC
    /// - that it considers this VC as invalid
    ///
    /// # Arguments
    ///
    /// * `vc_id` - vc_id to check document for
    /// * `value` - value to check
    pub async fn check_vc(&self, vc_id: &str, value: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Gets document for given vc name.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to fetch
    async fn get_vc_document(&self, vd_id: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// Sets document for given vc name.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to set value for
    /// * `value` - value to set
    async fn set_vc_document(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>>;
}