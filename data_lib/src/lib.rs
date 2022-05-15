//! This crate defines Enums, Structs, and Traits that you can use to persist
//! data in Foundation. It currently only depends on the Prelude
mod data_serialization {
    /// This enum provides a way to tag a Consumable as a certain type of data
    /// so that later, we can define parsing functions for each format
    // TODO: complete the enum
    pub enum DataFormat {
        CSV,
    }
    /// Consumable is a "package" for data that defines a common interface
    /// for reading and writing
    // All data IO is parsed to a Consumable before it is read to the binary
    // or written to storage
    pub struct Consumable {
        data: Vec<String>,
    }

    impl Consumable {
        pub fn new(dataset: Vec<String>) -> Consumable {
            Consumable { data: dataset }
        }
        pub fn get_data(&self) -> &Vec<String> {
            &self.data
        }
    }
}
/// This crate defines clients to backend storage.
/// All clients implement a common interface, DataClient
/// So that data can be accessed and manipulated in the same way
/// Data clients
mod data_clients {
    use crate::data_serialization::{Consumable, DataFormat};
    use std::path;

    use postgres::{Client, Error, NoTls, Row};

    pub trait DataClient {
        type ConsumableType;
        fn get_data_by_title(&mut self) -> Option<Consumable>;
        fn get_consumable_from_data(
            &mut self,
            dataset: Vec<Self::ConsumableType>,
        ) -> Consumable;
        fn import_data(&mut self, data: Consumable);
    }
    pub struct PostgresSQLClient {
        user_name: String,
        pass_word: String,
        host_name: String,
        data_base: String,
        port: String,
        client: Client,
    }

    impl PostgresSQLClient {
        pub fn new(
            username: &str,
            password: &str,
            hostname: &str,
            database: &str,
        ) -> Option<PostgresSQLClient> {
            if let Ok(client) = Client::connect(
                format!(
                    "postgresql://{}:{}@{}/{}",
                    username.to_string(),
                    password.to_string(),
                    hostname.to_string(),
                    database.to_string(),
                )
                .as_str(),
                NoTls,
            ) {
                Some(PostgresSQLClient {
                    user_name: username.to_string(),
                    pass_word: password.to_string(),
                    host_name: hostname.to_string(),
                    data_base: database.to_string(),
                    port: "".to_string(),
                    client,
                })
            } else {
                None
            }
        }

        fn rows_to_string(rows: Vec<Row>) -> Vec<String> {
            let mut rows_as_strings: Vec<String> = Vec::new();
            for row in rows {

                // TODO: Unimplemented
            }
            rows_as_strings
        }

        fn make_table_from_consumable(&mut self, table: Consumable) {
            let query =
                PostgresSQLClient::make_table_query_string(table.get_data());
            self.client.execute(&query, &[]).unwrap();
        }

        pub fn make_table_query_string(rows: &Vec<String>) -> String {
            let prefix = "CREATE TABLE ".to_owned();
            let postfix = " )".to_owned();
            let first_argument = &rows[0];
            let column_names_and_types = rows[1..].join(",").to_owned();
            let query = prefix
                + &format!("{}{}", &first_argument, "(".to_string())
                + &column_names_and_types
                + &postfix;
            query
        }
    }

    impl DataClient for PostgresSQLClient {
        type ConsumableType = Row;
        fn get_data_by_title(&mut self) -> Option<Consumable> {
            if let Ok(dataset) =
                self.client.query("SELECT * FROM materials_list", &[])
            {
                Some(self.get_consumable_from_data(dataset))
            } else {
                None
            }
        }
        fn get_consumable_from_data(
            &mut self,
            dataset: Vec<Row>,
        ) -> Consumable {
            let consumable_data = PostgresSQLClient::rows_to_string(dataset);
            Consumable::new(consumable_data)
        }

        fn import_data(&mut self, data: Consumable) {
            self.make_table_from_consumable(data)
        }
    }
}

mod data_loader {
    use postgres::{Client, NoTls};

    pub mod material_loader {
        use postgres::{Client, Error, Row};
        pub fn load_materials_list(mut client: Client) {
            for row in &client.query("SELECT * from materials", &[]).unwrap() {
                let id: i32 = row.get(0);
                let name: &str = row.get(1);
            }
        }
    }
}

#[cfg(test)]
mod datalib_tests {
    /* USING */
    use crate::data_clients::PostgresSQLClient;
    use crate::data_loader::material_loader::load_materials_list;
    use postgres::{Client, NoTls};

    #[test]
    fn postgres_parameter_parsing_does_produce_valid_ddl() {
        let column_names_and_types = vec![
            "test ".to_string(),
            " id SERIAL PRIMARY KEY".to_string(),
            " dummy_column VARCHAR NOT NULL".to_string(),
        ];

        let ddl_statement =
            PostgresSQLClient::make_table_query_string(&column_names_and_types);
        assert_eq!(
            "CREATE TABLE test ( id SERIAL PRIMARY KEY, dummy_column VARCHAR NOT NULL )",
            ddl_statement,
            "ddl is not \
        valid"
        );
    }

    #[test]
    fn can_connect_to_backend() {
        if let Some(v) = PostgresSQLClient::new(
            "postgres",
            "postgres",
            "localhost",
            "practice",
        ) {
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
