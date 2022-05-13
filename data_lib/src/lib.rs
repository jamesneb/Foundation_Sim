mod data_serialization {
    pub enum DataFormat {
        CSV,
    }

    pub struct Consumable {
        data: Vec<String>,
    }

    impl Consumable {
        pub fn new(dataset: Vec<String>) -> Consumable {
            Consumable { data: dataset }
        }
        pub fn get_data(&self) -> Vec<String> {
            self.data
        }
    }
}

mod data_clients {
    use crate::data_serialization::{Consumable, DataFormat};

    use postgres::{Client, Error, NoTls, Row};

    pub trait DataClient {
        fn get_data_by_title(&mut self) -> Option<Consumable>;
        fn get_consumable_from_data<T>(&mut self, dataset: Vec<T>) -> Consumable;
        fn import_data(&mut self, data: Consumable);
    }

    pub struct PostGresSqlClient {
        user_name: String,
        host_name: String,
        port: String,
        client: Client,
    }

    impl PostGresSqlClient {
        pub fn new(username: &str, hostname: &str, port: &str) -> Option<PostGresSqlClient> {
            if let Ok(client) = Client::connect(
                format!(
                    "postgresql://{}@{}:{}",
                    username.to_string(),
                    hostname.to_string(),
                    port.to_string()
                )
                .as_str(),
                NoTls,
            ) {
                Some(PostGresSqlClient {
                    user_name: username.to_string(),
                    host_name: hostname.to_string(),
                    port: port.to_string(),
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
            let query = self.make_table_query_string(table.get_data());
            self.client.execute(query, &[]).unwrap();
        }

        fn make_table_query_string(&mut self, rows: Vec<String>) -> &str {
            let prefix = "CREATE TABLE (".to_owned();
            let postfix = ")".to_owned();
            let column_names_and_types = rows.join(",").strip_suffix(",").unwrap().to_owned();
            let query = prefix + &column_names_and_types + &postfix;
            &query
        }
    }

    impl DataClient for PostGresSqlClient {
        fn get_data_by_title(&mut self) -> Option<Consumable> {
            if let Ok(dataset) = self.client.query("SELECT * FROM materials_list", &[]) {
                Some(self.get_consumable_from_data(dataset))
            } else {
                None
            }
        }
        fn get_consumable_from_data<T>(&mut self, dataset: Vec<T>) -> Consumable {
            let consumable_data = PostGresSqlClient::rows_to_string(dataset);
            Consumable::new(consumable_data)
        }

        fn import_data(&mut self, data: Consumable) {
            self.make_table_from_consumable(data)
        }
    }

    impl Drop for PostGresSqlClient {
        fn drop(&mut self) {
            self.client.close();
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
mod backend_tests {
    /* USING */
    use crate::data_loader::material_loader::load_materials_list;
    use postgres::{Client, NoTls};

    #[test]
    fn did_receive_materials_list() {}
    #[test]
    fn can_connect_to_backend() {
        let client = Client::connect("host=localhost user=postgresql", NoTls);

        assert!(client.is_err(), "unable to connect to database");

        let connection = match client {
            Ok(v) => v.close(),
            Err(e) => Err(e),
        };

        assert!(connection.is_err(), "unable to disconnect from database");
    }
}
