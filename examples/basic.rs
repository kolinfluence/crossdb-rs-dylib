use std::ffi::{CStr, CString};

// Import the bindings from the crate
use crossdb_rs_dylib::bindings::{
    xdb_open, xdb_exec, xdb_free_result, xdb_begin, xdb_commit, xdb_rollback, xdb_close, xdb_version,
    xdb_column_int, xdb_column_str, xdb_fetch_row, xdb_row_t,
};

fn main() {
    unsafe {
        println!("Starting CrossDB example");

        // Open a connection to CrossDB
        let c = CString::new("./db").unwrap();
        let conn = xdb_open(c.as_ptr() as *const i8);
        if conn.is_null() {
            panic!("Failed to open CrossDB connection");
        }
        println!("Connection opened successfully");

        // Create a table
        let create_table = CString::new("CREATE TABLE users (id INT PRIMARY KEY, name CHAR(50), age INT)").unwrap();
        let result = xdb_exec(conn, create_table.as_ptr() as *const i8);
        if result.is_null() {
            panic!("Failed to create table");
        }
        xdb_free_result(result);
        println!("Table created successfully");

        // Insert some data
        let insert_data = CString::new("INSERT INTO users (id, name, age) VALUES (1, 'Alice', 30), (2, 'Bob', 25), (3, 'Charlie', 35)").unwrap();
        let result = xdb_exec(conn, insert_data.as_ptr() as *const i8);
        if result.is_null() {
            panic!("Failed to insert data");
        }
        xdb_free_result(result);
        println!("Data inserted successfully");

        // Query the data
        let select_all = CString::new("SELECT * FROM users").unwrap();
        let res = xdb_exec(conn, select_all.as_ptr() as *const i8);
        if res.is_null() {
            panic!("Failed to query data");
        }
        println!("Query executed successfully");

        // Print the results
        println!("All users:");
        let mut row: *mut xdb_row_t;
        while {row = xdb_fetch_row(res); !row.is_null()} {
            println!("Fetched row: {:?}", row);
            
            // Check if the result set is valid
            if res.is_null() {
                println!("Error: Result set is null");
                break;
            }

            // Fetch and print each column with error checking
            for i in 0..3 {
                match i {
                    0 | 2 => {
                        let value = xdb_column_int(res as u64, row, i);
                        println!("Column {}: {}", i, value);
                    },
                    1 => {
                        let ptr = xdb_column_str(res as u64, row, i);
                        if ptr.is_null() {
                            println!("Column {}: NULL", i);
                        } else {
                            let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                            println!("Column {}: {}", i, value);
                        }
                    },
                    _ => unreachable!(),
                }
            }

            println!("---");
        }
        xdb_free_result(res);
        println!("Results printed and freed");

        // Update a record
        let update_record = CString::new("UPDATE users SET age = 31 WHERE id = 1").unwrap();
        let result = xdb_exec(conn, update_record.as_ptr() as *const i8);
        if result.is_null() {
            panic!("Failed to update record");
        }
        xdb_free_result(result);

        // Query a specific user
        let select_one = CString::new("SELECT * FROM users WHERE id = 1").unwrap();
        let res = xdb_exec(conn, select_one.as_ptr() as *const i8);
        if res.is_null() {
            panic!("Failed to query specific user");
        }

        println!("\nUpdated user:");
        let row = xdb_fetch_row(res);
        if !row.is_null() {
            println!("Fetched row: {:?}", row);
            
            // Check if the result set is valid
            if res.is_null() {
                println!("Error: Result set is null");
            } else {
                // Fetch and print each column with error checking
                for i in 0..3 {
                    match i {
                        0 | 2 => {
                            let value = xdb_column_int(res as u64, row, i);
                            println!("Column {}: {}", i, value);
                        },
                        1 => {
                            let ptr = xdb_column_str(res as u64, row, i);
                            if ptr.is_null() {
                                println!("Column {}: NULL", i);
                            } else {
                                let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                                println!("Column {}: {}", i, value);
                            }
                        },
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            println!("No row fetched");
        }
        xdb_free_result(res);
        println!("Result freed");

        // Delete a record
        let delete_record = CString::new("DELETE FROM users WHERE id = 2").unwrap();
        let result = xdb_exec(conn, delete_record.as_ptr() as *const i8);
        if result.is_null() {
            panic!("Failed to delete record");
        }
        xdb_free_result(result);

        // Query all users again
        let select_all = CString::new("SELECT * FROM users").unwrap();
        let res = xdb_exec(conn, select_all.as_ptr() as *const i8);
        if res.is_null() {
            panic!("Failed to query data");
        }

        println!("\nRemaining users:");
        let mut row: *mut xdb_row_t;
        while {row = xdb_fetch_row(res); !row.is_null()} {
            println!("Fetched row: {:?}", row);
            
            // Check if the result set is valid
            if res.is_null() {
                println!("Error: Result set is null");
                break;
            }

            // Fetch and print each column with error checking
            for i in 0..3 {
                match i {
                    0 | 2 => {
                        let value = xdb_column_int(res as u64, row, i);
                        println!("Column {}: {}", i, value);
                    },
                    1 => {
                        let ptr = xdb_column_str(res as u64, row, i);
                        if ptr.is_null() {
                            println!("Column {}: NULL", i);
                        } else {
                            let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                            println!("Column {}: {}", i, value);
                        }
                    },
                    _ => unreachable!(),
                }
            }
            println!("---");
        }
        println!("Finished fetching rows");
        xdb_free_result(res);
        println!("Results freed");

        // Demonstrate transaction
        let status = xdb_begin(conn);
        if status != 0 {
            panic!("Failed to begin transaction");
        }

        let insert_transaction = CString::new("INSERT INTO users (id, name, age) VALUES (4, 'David', 40)").unwrap();
        let result = xdb_exec(conn, insert_transaction.as_ptr() as *const i8);
        if result.is_null() {
            xdb_rollback(conn);
            panic!("Failed to insert data in transaction");
        }
        xdb_free_result(result);

        let status = xdb_commit(conn);
        if status != 0 {
            panic!("Failed to commit transaction");
        }

        // Verify the transaction
        let select_transaction = CString::new("SELECT * FROM users WHERE id = 4").unwrap();
        let res = xdb_exec(conn, select_transaction.as_ptr() as *const i8);
        if res.is_null() {
            panic!("Failed to query transaction data");
        }

        println!("\nUser added in transaction:");
        let row = xdb_fetch_row(res);
        if !row.is_null() {
            println!("Fetched row: {:?}", row);
            
            // Check if the result set is valid
            if res.is_null() {
                println!("Error: Result set is null");
            } else {
                // Fetch and print each column with error checking
                for i in 0..3 {
                    match i {
                        0 | 2 => {
                            let value = xdb_column_int(res as u64, row, i);
                            println!("Column {}: {}", i, value);
                        },
                        1 => {
                            let ptr = xdb_column_str(res as u64, row, i);
                            if ptr.is_null() {
                                println!("Column {}: NULL", i);
                            } else {
                                let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                                println!("Column {}: {}", i, value);
                            }
                        },
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            println!("No row fetched");
        }
        println!("Freeing result");
        xdb_free_result(res);
        println!("Result freed");

        let version = CStr::from_ptr(xdb_version()).to_string_lossy().into_owned();
        println!("\nCrossDB Version: {}", version);

        // Close the connection
        xdb_close(conn);
    }
}
