use std::path::PathBuf;

use warp_common::chrono::{DateTime, Utc};
use warp_common::serde::{Deserialize, Serialize};
use warp_common::{Extension, Module};
use warp_constellation::directory::Directory;
use warp_constellation::file::File;
use warp_constellation::Constellation;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "warp_common::serde")]
pub struct DummyFileSystem {
    index: Directory,
    modified: DateTime<Utc>,
    path: PathBuf,
}

impl Default for DummyFileSystem {
    fn default() -> Self {
        DummyFileSystem {
            index: Directory::new("root"),
            modified: Utc::now(),
            path: PathBuf::new(),
        }
    }
}

impl Extension for DummyFileSystem {
    fn id(&self) -> String {
        String::from("test")
    }

    fn name(&self) -> String {
        "Dummy Filesystem".to_string()
    }

    fn module(&self) -> Module {
        Module::FileSystem
    }
}

impl Constellation for DummyFileSystem {
    fn modified(&self) -> DateTime<Utc> {
        self.modified
    }

    fn root_directory(&self) -> &Directory {
        &self.index
    }

    fn root_directory_mut(&mut self) -> &mut Directory {
        &mut self.index
    }

    fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    fn get_path(&self) -> &PathBuf {
        &self.path
    }

    fn get_path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }
}

fn main() -> warp_common::Result<()> {
    let mut dummy_fs = DummyFileSystem::default();
    let mut file = File::new("testFile.png");
    file.set_size(10000);

    let mut directory = Directory::new("Test Directory");
    directory.add_item(file.clone())?;

    let mut new_directory = Directory::new("Test Directory");
    new_directory.add_item(file)?;

    directory.add_item(new_directory)?;

    dummy_fs.root_directory_mut().add_item(directory)?;

    Ok(())
}
