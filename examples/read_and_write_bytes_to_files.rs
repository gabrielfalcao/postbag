use color_eyre::Report;
use iocore::Path;
use postbag::{from_full_slice, from_slim_slice, to_full_vec, to_slim_vec};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub fn serialized_location_path<T: std::fmt::Display>(filename: T) -> Path {
    let filename = filename.to_string();
    Path::new(file!()).parent().unwrap().join(&filename)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
    active: bool,
}
impl Person {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum SerializationType {
    Full,
    Slim,
}
impl FromStr for SerializationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim().to_lowercase().to_string();
        match input.as_str() {
            "full" => Ok(SerializationType::Full),
            "slim" => Ok(SerializationType::Slim),
            thing => Err(format!("unrecognized serializazation type: {thing:#?}")),
        }
    }
}
impl std::fmt::Display for SerializationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SerializationType::Full => "full",
                SerializationType::Slim => "slim",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SerializedInfo {
    name: String,
    // ty: SerializationType,
    // path: Path,
    // data: Vec<u8>,
    person: Person,
}

impl SerializedInfo {
    fn new(person: &Person) -> Result<SerializedInfo, Report> {
        let person = person.clone();
        let name = person.name().to_lowercase();
        // let data = to_full_vec(&person).unwrap();
        // path.write(&data)?;

        Ok(SerializedInfo {
            name,
            // ty,
            // path,
            // data,
            person,
        })
    }
    pub fn path(&self, ty: SerializationType) -> Path {
        let name = self.name.to_string();
        let filename = format!("{name}-{ty}.bin");
        let path = serialized_location_path(&filename);
        path
    }
    pub fn serialize(&self, ty: SerializationType) -> Result<Vec<u8>, Report> {
        Ok(match ty {
            SerializationType::Full => to_full_vec(&self.person)?,
            SerializationType::Slim => to_slim_vec(&self.person)?,
        })
    }
    fn deserialize<T>(&self, ty: SerializationType) -> Result<Person, Report> {
        let bytes = self.serialize(ty)?;
        let result = match ty {
            SerializationType::Full => from_full_slice(&bytes)?,
            SerializationType::Slim => from_slim_slice(&bytes)?,
        };
        Ok(result)
    }
}

fn main() -> Result<(), Report> {
    let alice_person = Person { name: "Alice".to_string(), age: 30, active: true };
    let alice_info = SerializedInfo::new(&alice_person)?;

    let slim_bytes = alice_info.serialize(SerializationType::Slim)?;
    let person_from_slim_bytes: Person = alice_info.deserialize::<Person>(SerializationType::Slim)?;
    println!("✓ Person serialized/deserialized via slim: {} bytes", slim_bytes.len());


    let full_bytes = alice_info.serialize(SerializationType::Full)?;
    let _person_from_full_bytes: Person = alice_info.deserialize::<Person>(SerializationType::Full)?;
    println!("✓ Person serialized/deserialized via full: {} bytes", full_bytes.len());

    assert_eq!(alice_person, person_from_slim_bytes);

    println!("✓ Space saved with slim: {} bytes", full_bytes.len() - slim_bytes.len());
    Ok(())
}
