mod error;
mod storage;

pub use error::*;
pub use storage::AccessStorage;
pub use storage::fs::FileStorage;

#[cfg(test)]
mod tests {
    use temp_testdir::TempDir;
    use tokio_test::block_on;
    use crate::AccessStorage;
    use crate::storage::fs::FileStorage;

    #[test]
    fn test_fs_storage() {
        const TEST_BYTES: [u8; 4] = [0xde, 0xad, 0xfa, 0xce];

        let dir = TempDir::default();
        let sto = FileStorage::new(dir.as_ref());

        block_on(async {
            sto.write_bytes("foo", &TEST_BYTES).await.unwrap();

            let lst_iter = sto.list("").await.unwrap().into_iter();
            let lst: Vec<_> = lst_iter.collect();
            println!("lst = {:?}", lst);
            assert_eq!(1, lst.len());
            assert_eq!("foo", lst[0]);

            // check illegal object names don't work
            assert!(sto.list("/").await.is_err());
            assert!(sto.list("hello/world").await.is_err());
        });

    }
}
