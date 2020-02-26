pub mod traits;

use futures::future::{ select_ok, try_join_all };
use simple_error::SimpleError;
use traits::{ DidResolver, Logger, VcResolver };

//// Core of library.
pub struct Library {
    /// Vector of supported DID resolvers.
    pub did_resolvers: Vec<Box<dyn DidResolver>>,
    /// Vector of supported loggers. Logging will iterate through it and try to use every logger.
    pub loggers: Vec<Box<dyn Logger>>,
    /// Vector of supported VC resolvers.
    pub vc_resolvers: Vec<Box<dyn VcResolver>>,
}

impl Library {
    /// Creates new Library instance, vectors are initialized as empty.
    pub fn new() -> Library {
        Library {
            did_resolvers: Vec::new(),
            loggers: Vec::new(),
            vc_resolvers: Vec::new(),
        }
    }

    /// Gets document for given did name.
    /// If multiple plugins are registered, first **successful** response
    /// will be used. Request will fail if all plugins failed.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to fetch
    pub async fn get_did_document(&self, did_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let futures = self.did_resolvers.iter()
            .map(|resolver| resolver.get_did_document(did_name));
        match select_ok(futures).await {
            Ok((r, _)) => Ok(r),
            Err(_e) => Err(Box::new(SimpleError::new(format!("could not get did document")))),
        }
    }

    /// Log given message. Logging will iterate through it and try to use every logger.
    ///
    /// # Arguments
    ///
    /// * `message` - message to log
    pub fn log(&self, message: &str, level: Option<&str>) {
        for logger in self.loggers.iter() {
            logger.log(message, level);
        }
    }

    /// Registers new `DidResolver` instance. Note, that `did_resolver` is given as `Box` to support dynamic assignment.
    ///
    /// # Arguments
    /// 
    /// * `did_resolver` - an instance of a `struct` that implements `DidResolver` trait
    pub fn register_did_resolver(&mut self, did_resolver: Box<dyn DidResolver>) {
        self.did_resolvers.push(did_resolver);
    }

    /// Registers new `Logger` instance. Note, that `logger` is given as `Box` to support dynamic assignment.
    ///
    /// # Arguments
    /// 
    /// * `logger` - an instance of a `struct` that implements `Logger` trait
    pub fn register_logger(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }

    /// Registers new `VcdResolver` instance. Note, that `vc_resolver` is given as `Box` to support dynamic assignment.
    ///
    /// # Arguments
    /// 
    /// * `vc_resolver` - an instance of a `struct` that implements `VcResolver` trait
    pub fn register_vc_resolve(&mut self, vc_resolver: Box<dyn VcResolver>) {
        self.vc_resolvers.push(vc_resolver);
    }

    /// Sets document for given did name.
    /// If multiple plugins are registered, awaits completion of all actions.
    /// First plugin, that fails lets this request fail.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to set value for
    /// * `value` - value to set
    pub async fn set_did_document(&mut self, did_name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let futures = self.did_resolvers.iter_mut()
            .map(|resolver| resolver.set_did_document(did_name, value));
        match try_join_all(futures).await {
            Ok(_) => Ok(()),
            Err(_e) => Err(Box::new(SimpleError::new(format!("could not set did document")))),
        }
    }
}
