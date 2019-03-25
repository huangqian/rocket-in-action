#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::{Rocket, State};
use rocket_contrib::json::{Json, JsonValue};

type ID = usize;
type PersonMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Person {
    id: Option<ID>,
    name: String,
}

/**
 * 创建一个用户
 */
#[post("/<id>", format = "json", data = "<person>")]
fn create(id: ID, person: Json<Person>, map: State<PersonMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
        "status": "error",
        "message": "id exists, Try put."
        })
    } else {
        hashmap.insert(id, person.0.name);
        json!({"status": "ok"})
    }
}

#[put("/<id>", format = "json", data = "<person>")]
fn update(id: ID, person: Json<Person>, map: State<PersonMap>) -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, person.0.name);
        json!({"status": "ok"})
    } else {
        json!({"status": "error", "message": "id not exists!"})
    }
}

#[get("/<id>", format = "json")]
fn get(id: ID, map: State<PersonMap>) -> Json<Person> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|name| {
        Json(Person {
            id: Some(id),
            name: name.clone(),
        })
    }).unwrap()
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
    "status": "error",
    "message": "resource was not found."
    })
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/person", routes![create, update, get])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    use super::rocket;

    #[test]
    pub fn test_post() {
        let client = Client::new(rocket()).unwrap();

        let mut response = client
            .post("/person/1")
            .header(ContentType::JSON)
            .body(r#"{"id": 1, "name": "jerry" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        println!("{}", response.body_string().unwrap());
    }

    #[test]
    pub fn test_put(){

        let client = Client::new(rocket()).unwrap();
        let mut res = client
            .put("/person/1")
            .header(ContentType::JSON)
            .body(r#"{"id": 1, "name": "kafka"}"#)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        println!("{}", res.body_string().unwrap());
    }
}

