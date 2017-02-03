use std::collections::HashMap;

use config::{Result, Config, Value, Environment};
use config::toml_ext::IntoValue;
use logger::LoggingLevel;

/// Structure following the builder pattern for building `Config` structures.
#[derive(Clone)]
pub struct ConfigBuilder {
    /// The environment that this configuration corresponds to.
    pub environment: Environment,
    /// The address to serve on.
    pub address: String,
    /// The port to serve on.
    pub port: u16,
    /// The number of workers to run in parallel.
    pub workers: u16,
    /// How much information to log.
    pub log_level: LoggingLevel,
    /// The session key.
    pub session_key: Option<String>,
    /// Any extra parameters that aren't part of Rocket's config.
    pub extras: HashMap<String, Value>,
}

impl ConfigBuilder {
    /// Create a new `ConfigBuilder` instance using the default parameters from
    /// the given `environment`. The root configuration directory is set to the
    /// current working directory.
    ///
    /// This method is typically called indirectly via
    /// [Config::build](/rocket/config/struct.Config.html#method.build).
    ///
    /// # Panics
    ///
    /// Panics if the current directory cannot be retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .port(700)
    ///     .workers(12)
    ///     .finalize();
    ///
    /// # assert!(config.is_ok());
    /// ```
    pub fn new(environment: Environment) -> ConfigBuilder {
        let config = Config::new(environment)
            .expect("ConfigBuilder::new(): couldn't get current directory.");

        ConfigBuilder {
            environment: config.environment,
            address: config.address,
            port: config.port,
            workers: config.workers,
            log_level: config.log_level,
            session_key: None,
            extras: config.extras,
        }
    }

    /// Sets the `address` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .unwrap();
    ///
    /// assert_eq!(config.address.as_str(), "127.0.0.1");
    /// ```
    pub fn address<A: Into<String>>(mut self, address: A) -> Self {
        self.address = address.into();
        self
    }

    /// Sets the `port` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .port(1329)
    ///     .unwrap();
    ///
    /// assert_eq!(config.port, 1329);
    /// ```
    #[inline]
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets `workers` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .workers(64)
    ///     .unwrap();
    ///
    /// assert_eq!(config.workers, 64);
    /// ```
    #[inline]
    pub fn workers(mut self, workers: u16) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the `log_level` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::LoggingLevel;
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .log_level(LoggingLevel::Critical)
    ///     .unwrap();
    ///
    /// assert_eq!(config.log_level, LoggingLevel::Critical);
    /// ```
    #[inline]
    pub fn log_level(mut self, log_level: LoggingLevel) -> Self {
        self.log_level = log_level;
        self
    }

    /// Sets the `session_key` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::LoggingLevel;
    /// use rocket::config::{Config, Environment};
    ///
    /// let key = "VheMwXIBygSmOlZAhuWl2B+zgvTN3WW5";
    /// let mut config = Config::build(Environment::Staging)
    ///     .session_key(key)
    ///     .unwrap();
    ///
    /// assert_eq!(config.take_session_key(), Some(key.to_string()));
    /// ```
    pub fn session_key<K: Into<String>>(mut self, key: K) -> Self {
        self.session_key = Some(key.into());
        self
    }

    /// Sets the `environment` in the configuration being built.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .environment(Environment::Production)
    ///     .unwrap();
    ///
    /// assert_eq!(config.environment, Environment::Production);
    /// ```
    #[inline]
    pub fn environment(mut self, env: Environment) -> Self {
        self.environment = env;
        self
    }

    /// Adds an extra configuration parameter with `name` and `value` to the
    /// configuration being built. The value can be any type that implements
    /// [IntoValue](/config/trait.IntoValue.html) including `&str`, `String`,
    /// `Vec<V: IntoValue>`, `HashMap<S: Into<String>, V: IntoValue>`, and all
    /// integer and float types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .extra("pi", 3.14)
    ///     .extra("custom_dir", "/a/b/c")
    ///     .unwrap();
    ///
    /// assert_eq!(config.get_float("pi"), Ok(3.14));
    /// assert_eq!(config.get_str("custom_dir"), Ok("/a/b/c"));
    /// ```
    pub fn extra<V: IntoValue>(mut self, name: &str, value: V) -> Self {
        self.extras.insert(name.into(), value.into_value());
        self
    }

    /// Return the `Config` structure that was being built by this builder.
    ///
    /// # Errors
    ///
    /// If the current working directory cannot be retrieved, returns a `BadCWD`
    /// error. If the address or session key fail to parse, returns a `BadType`
    /// error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .port(700)
    ///     .workers(12)
    ///     .finalize();
    ///
    /// assert!(config.is_ok());
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("?")
    ///     .finalize();
    ///
    /// assert!(config.is_err());
    /// ```
    pub fn finalize(self) -> Result<Config> {
        let mut config = Config::new(self.environment)?;
        config.set_address(self.address)?;
        config.set_port(self.port);
        config.set_workers(self.workers);
        config.set_log_level(self.log_level);
        config.set_extras(self.extras);

        if let Some(key) = self.session_key {
            config.set_session_key(key)?;
        }

        Ok(config)
    }

    /// Return the `Config` structure that was being built by this builder.
    ///
    /// # Panics
    ///
    /// Panics if the current working directory cannot be retrieved or if the
    /// supplied address or session key fail to parse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::{Config, Environment};
    ///
    /// let config = Config::build(Environment::Staging)
    ///     .address("127.0.0.1")
    ///     .unwrap();
    ///
    /// assert_eq!(config.address.as_str(), "127.0.0.1");
    /// ```
    #[inline(always)]
    pub fn unwrap(self) -> Config {
        self.finalize().expect("ConfigBuilder::unwrap() failed")
    }
}