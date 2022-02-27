use std::collections::HashMap;

use warp_common::error::Error;
use warp_common::Result;

use warp_data::DataObject;
use warp_module::Module;

/// `Hook` contains identifying information about a given hook.
#[derive(Debug, Clone, PartialEq)]
pub struct Hook {
    pub name: String,
    pub module: Module,
}

impl<A> From<A> for Hook
where
    A: AsRef<str>,
{
    fn from(hook: A) -> Self {
        let mut hook = hook.as_ref().split("::");
        let (module_name, name) = (
            hook.next().unwrap_or_default(),
            hook.next().unwrap_or_default().to_string(),
        );
        let module = Module::from(module_name);
        Hook { name, module }
    }
}

/// Allows for creation of a new hook
impl Hook {
    pub fn new<S: AsRef<str>>(name: S, module: Module) -> Self {
        let name = name.as_ref().to_string();
        Self { name, module }
    }
}

/// Formats the Hook into a unique, human readable, identifier
impl ToString for Hook {
    fn to_string(&self) -> String {
        format!("{}::{}", self.module, self.name)
    }
}

/// Wraps the data from a given hook in the standard DataObject
pub type HookData = Box<dyn Fn(Hook, DataObject)>;

/// Lists all of the hooks registered
#[derive(Default)]
pub struct Hooks {
    pub hooks: Vec<Hook>,
    pub subscribers: HashMap<String, Vec<HookData>>,
}

/// General methods for using hooks throughout the platform.
impl Hooks {
    /// Create a new `Hook` registered to the system.
    ///
    /// # Examples
    ///
    /// ```
    ///     use warp_data::DataObject;
    ///     use warp_hooks::hooks::{Hook, Hooks};
    ///     use warp_module::Module;
    ///
    ///     let mut system = Hooks::default();
    ///     let hook = system.create("NEW_FILE", Module::FileSystem).unwrap();
    ///     assert_eq!(hook, Hook::new("NEW_FILE", Module::FileSystem));
    /// ```
    pub fn create<S: AsRef<str>>(&mut self, name: S, module: Module) -> Result<Hook> {
        let name = name.as_ref().to_owned();
        let hook = Hook::new(name, module);

        if self.hooks.contains(&hook) {
            return Err(Error::DuplicateHook);
        }

        self.hooks.push(hook.clone());

        Ok(hook)
    }

    /// Get a list of hooks currently active in the system
    ///
    /// # Examples
    ///
    /// ```
    ///     use warp_data::DataObject;
    ///     use warp_hooks::hooks::{Hook, Hooks};
    ///     use warp_module::Module;
    ///
    ///     let mut system = Hooks::default();
    ///     let hook = system.create("NEW_FILE", Module::FileSystem).unwrap();
    ///     let hooks = system.hooks();
    ///     assert_eq!(hooks.get(0).unwrap(), &hook);
    /// ```
    pub fn hooks(&self) -> Vec<Hook> {
        self.hooks.clone()
    }

    /// Subscribe to events on a given hook
    ///
    /// # Examples
    ///
    /// ```
    ///     use warp_data::DataObject;
    ///     use warp_hooks::hooks::{Hook, Hooks};
    ///     use warp_module::Module;
    ///     use warp_constellation::file::File;
    ///
    ///     let mut system = Hooks::default();
    ///     let hook = system.create("NEW_FILE", Module::FileSystem).unwrap();
    ///     //Check to see if hook already exist
    ///     assert_eq!(system.create("NEW_FILE", Module::FileSystem).is_err(), true);
    ///     //Check to see if hook havent been registered
    ///     assert_eq!(system.subscribe(Hook::from("UNKNOWN::NEW_FILE"), |_, _|{}).is_err(), true);
    ///     system.subscribe("FILESYSTEM::NEW_FILE", |hook, data| {
    ///         assert_eq!(hook.name.as_str(), "NEW_FILE");
    ///         assert_eq!(hook.module, Module::FileSystem);
    ///         assert_eq!(data.module, Module::FileSystem);
    ///         let file: File = data.payload().unwrap();
    ///         assert_eq!(file.name.as_str(), "test.txt");
    ///     }).unwrap();
    ///     let data = DataObject::new(&Module::FileSystem, File::new("test.txt")).unwrap();
    ///     system.trigger("FILESYSTEM::NEW_FILE", "FILESYSTEM::NEW_FILE", &data);
    /// ```
    pub fn subscribe<C, H>(&mut self, hook: H, f: C) -> Result<()>
    where
        C: 'static + Fn(Hook, DataObject),
        H: Into<Hook>,
    {
        let hook = hook.into();
        if !self.hooks.contains(&hook) {
            return Err(Error::HookUnregistered);
        }
        self.subscribers
            .entry(hook.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(f));
        Ok(())
    }

    /// Trigger a hook to all subscribers
    ///
    /// # Examples
    ///
    /// ```
    ///     use warp_data::DataObject;
    ///     use warp_hooks::hooks::{Hook, Hooks};
    ///     use warp_module::Module;
    ///     use warp_constellation::file::File;
    ///
    ///     let mut system = Hooks::default();
    ///     let hook = system.create("NEW_FILE", Module::FileSystem).unwrap();
    ///     system.subscribe("FILESYSTEM::NEW_FILE", |hook, data| {
    ///         assert_eq!(hook.name.as_str(), "NEW_FILE");
    ///         assert_eq!(hook.module, Module::FileSystem);
    ///         assert_eq!(data.module, Module::FileSystem);
    ///         let file: File = data.payload().unwrap();
    ///         assert_eq!(file.name.as_str(), "test.txt");
    ///     }).unwrap();
    ///     let data = DataObject::new(&Module::FileSystem, File::new("test.txt")).unwrap();
    ///     system.trigger("FILESYSTEM::NEW_FILE", "FILESYSTEM::NEW_FILE", &data);
    /// ```
    pub fn trigger<S, H>(&self, name: S, hook: H, data: &DataObject)
    where
        S: AsRef<str>,
        H: Into<Hook>,
    {
        let hook = hook.into();
        if let Some(subscribers) = self.subscribers.get(name.as_ref()) {
            for subscriber in subscribers {
                subscriber(hook.clone(), data.clone());
            }
        }
    }
}
