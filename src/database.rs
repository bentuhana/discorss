#[allow(clippy::new_ret_no_self)]
use std::env;
use std::fs::{create_dir_all, read_dir};
use std::io::Error;
use std::path::{Path, PathBuf};

use pickledb::{PickleDb, PickleDbDumpPolicy};

pub struct Database;
impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<(), Error> {
        let mut database_file_path = PathBuf::new();
        database_file_path.push(db_path);
        let mut database_path = database_file_path.clone();
        database_path.pop();

        if read_dir(&database_path).is_err() {
            create_dir_all(&database_path)?
        }

        PickleDb::new_json(database_file_path, PickleDbDumpPolicy::AutoDump);
        Ok(())
    }
    pub fn load(dump_policy: Option<PickleDbDumpPolicy>) -> PickleDb {
        let database_file_path = env::var("DATABASE_FILE_PATH")
            .expect("Expected DATABASE_FILE_PATH environment variable.");

        PickleDb::load_json(
            database_file_path,
            dump_policy.unwrap_or(PickleDbDumpPolicy::AutoDump),
        )
        .unwrap()
    }
}
