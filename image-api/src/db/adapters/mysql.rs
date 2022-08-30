// use mysql::*;
// use mysql::prelude::*;

// pub fn print_data() {
//     let url = "mysql://root:Welcome1@localhost:3306/test";

//     let pool = Pool::new(url).expect("err");

//     let mut conn = pool.get_conn().expect("err");

//     let data = conn.query_map("SELECT * FROM TEST", |text:String| {text}).expect("err");

//     for dt in data {
//         println!("Data from DB: {}", dt);
//     }
// }
