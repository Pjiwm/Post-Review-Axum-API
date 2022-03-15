// use crate::models;
// use std::vec::Vec;
// static mut VALUES: Vec<models::User> = Vec::new();

// pub fn add(user: models::User) {
//     unsafe {
//         VALUES.push(user);
//     }
// }

// pub fn get() -> Vec<models::User> {
//     unsafe {
//         let mut list: Vec<models::User> = Vec::new();
//         for index in VALUES.iter() {
//             let copy = index.copy();
//             list.push(copy);
//         }
//         return list;
//     }
// }

// pub fn get_by_id(id: i64) -> Option<models::User> {
//     unsafe {
//         for index in VALUES.iter() {
//             if index.id == id {
//                 return Some(index.copy());
//             }
//         }
//         None
//     }
// }

// pub fn remove_by_id(id: i64) {
//     unsafe {
//         for (i, index) in VALUES.iter().enumerate() {
//             if index.id == id {
//                 VALUES.remove(i);
//                 return;
//             }
//         }
//     }
// }

// pub fn update(user: models::User, id: i64) -> bool {
//     unsafe {
//         for (i, index) in VALUES.iter().enumerate() {
//             if index.id == id {
//                 VALUES[i] = user.copy();
//                 return true;
//             }
//         }
//     }
//     return false;
// }
