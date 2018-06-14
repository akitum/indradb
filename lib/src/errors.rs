// TODO: remove this once error-chain fixes it
#![allow(renamed_and_removed_lints)]

#[cfg(feature = "rocksdb-datastore")]
use bincode::Error as BincodeError;
#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Json(JsonError);
        RocksDb(RocksDbError) #[cfg(feature = "rocksdb-datastore")];
        Bincode(BincodeError) #[cfg(feature = "rocksdb-datastore")];
    }
}

error_chain! {
    types {
        ValidationError, ValidationErrorKind, ValidationResultExt, ValidationResult;
    }
}
