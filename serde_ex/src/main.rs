use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Pay {
    amount: i32,
    tax_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Persion {
    name: String,
    age: u8,
    phone: String,
    pays: Vec<Pay>,
}

// use serde::ser::{Serialize, SerializeStruct};

// struct Persion {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }

// impl Serialize for Persion {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut s = serializer.serialize_struct("persion", 3)?;
//         s.serialize_field("name", &self.name)?;
//         s.serialize_field("age", &self.age)?;
//         s.serialize_field("phones", &self.phones)?;
//         s.end()
//     }
// }

fn main() {
    let ps = vec![Persion {
        name: "akita".to_string(),
        age: 26,
        phone: "777".to_string(),
        pays: vec![
            Pay {
                amount: 32,
                tax_percent: 0.3,
            },
            Pay {
                amount: 77,
                tax_percent: 0.2,
            },
        ],
    }];

    let json_str = serde_json::to_string_pretty(&ps).unwrap();
    let yaml_str = serde_yaml::to_string(&ps).unwrap();
    println!("json: {}", json_str);
    println!("yaml: {}", yaml_str);

    let ps_json: Vec<Persion> = serde_json::from_str(&json_str).unwrap();
    let ps_yaml: Vec<Persion> = serde_yaml::from_str(&yaml_str).unwrap();
    println!("ps_json: {:?}", ps_json);
    println!("ps_yaml: {:?}", ps_yaml);

    let json_data = std::fs::read_to_string("./data.json").unwrap();
    let mut data: serde_json::Value = serde_json::from_str(&json_data).unwrap();
    println!("json_data before: {:?}", data);
    data["car"] = serde_json::Value::String("fd".to_string());
    let mut map_value = serde_json::Map::new();
    map_value.insert(
        "color".to_string(),
        serde_json::Value::Array(vec![serde_json::Value::String("blue".to_string())]),
    );
    data["car_props"] = serde_json::Value::Object(map_value);
    println!("json_data after: {:?}", data);
}
