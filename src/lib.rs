mod error;
mod names;
mod storage;
mod lookup;
mod index;
mod indexer;

pub use error::*;
pub use names::*;
pub use storage::AccessStorage;
pub use storage::fs::FileStorage;
pub use lookup::Lookup;
pub use index::Index;
pub use indexer::hashtable_indexer::HashTableIndexer;

#[cfg(test)]
mod tests {
    use serde::{Serialize,Deserialize};
    use temp_testdir::TempDir;
    use tokio_test::block_on;
    use crate::{
        AccessStorage,
        ObjectName,
        ObjectNameBuf,
        HashTableIndexer,
        Index,
        Lookup
    };
    use crate::storage::fs::FileStorage;

    #[test]
    fn test_object_naming() {
        // check illegal object names don't work
        assert!(ObjectName::new("/").is_err());
        assert!(ObjectName::new("hello/world").is_err());
        assert!(ObjectName::new("foo.bar").is_ok());
        assert!(ObjectName::new("föö.bär").is_ok());
    }

    #[test]
    fn test_fs_storage() {
        const TEST_BYTES: [u8; 4] = [0xde, 0xad, 0xfa, 0xce];

        let dir = TempDir::default();
        let sto = FileStorage::new(dir.as_ref());

        block_on(async {
            let foo = ObjectName::new("foo").unwrap();
            sto.write_bytes(foo, &TEST_BYTES).await.unwrap();

            let lst_iter = sto.list(ObjectName::empty()).await.unwrap().into_iter();
            let lst: Vec<_> = lst_iter.collect();
            println!("lst = {:?}", lst);
            assert_eq!(1, lst.len());
            assert_eq!("foo", lst[0]);

            let rd_data = sto.read_bytes(foo).await.unwrap();
            assert_eq!(TEST_BYTES.len(), rd_data.len());
            assert_eq!(TEST_BYTES, rd_data[0..(TEST_BYTES.len())]);
        });
    }


    #[derive(Debug,Deserialize,Serialize,PartialEq)]
    struct TestData {
        name: String,
        blob: Vec<i32>,
    }

    #[test]
    fn test_json_objects() {
        let test_object = TestData {
            name: String::from("Hello World"),
            blob: vec![1, 1, -1312, 233, 585],
        };

        let dir = TempDir::default();
        let sto = FileStorage::new(dir.as_ref());

        block_on(async {
            let foo = ObjectName::new("foo.json").unwrap();
            sto.write_json(foo, &test_object).await.unwrap();

            let rd_obj: Box<TestData> = sto.read_json(foo).await.unwrap();
            assert_eq!(test_object, *rd_obj);
        });
    }

    #[derive(Debug,Deserialize,Serialize,PartialEq)]
    struct TestIndexData {
        number: i32
    }

    async fn index_by_name(_: &FileStorage, name: ObjectNameBuf) -> String {
        name.name().name().to_string()
    }

    async fn index_by_number(sto: &FileStorage, name: ObjectNameBuf) -> i32 {
        let res: Result<Box<TestIndexData>,_> = sto.read_json(name.name()).await;

        if let Ok(content) = res {
            content.number
        } else {
            -1
        }
    }

    #[test]
    fn test_indexer() {
        const FILENAMES: [&str; 4] = [
            "foo", "bar", "baz", "blub"
        ];
        const CONTENT: [i32; 4] = [
            1, 2, 3, 4
        ];

        let dir = TempDir::default();
        let sto = FileStorage::new(dir.as_ref());

        block_on(async {
            // prep directory
            for (filename, content) in FILENAMES.iter().zip(CONTENT.iter()) {
                let name = ObjectName::new(filename).unwrap();
                let obj = TestIndexData {
                    number: *content
                };

                sto.write_json(name, &obj).await.unwrap();
            }

            // create index
            let name_index = HashTableIndexer::index(&sto,
                                                     ObjectName::empty(),
                                                     index_by_name)
                .await.unwrap();

            let number_index = HashTableIndexer::index(&sto,
                                                       ObjectName::empty(),
                                                       index_by_number)
                .await.unwrap();

            // test index
            for filename in FILENAMES.iter() {
                let lkup = name_index.get(&filename.to_string()).unwrap();
                assert_eq!(vec![ObjectName::new(filename).unwrap()], lkup);
            }

            for (content,filename) in CONTENT.iter().zip(FILENAMES.iter()) {
                let lkup = number_index.get(content).unwrap();
                assert_eq!(vec![ObjectName::new(filename).unwrap()], lkup);
            }
        });
    }
}
