use crate::enums::Enum;
use crate::world::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[allow(unused_macros)]
macro_rules! from_old_version {
    ($(-$remove:ident)* $(+$add:ident)*) => {
        impl std::convert::TryFrom<World> for $crate::world::World {
            type Error = serde_json::Error;

            #[allow(unused)]
            fn try_from(value: World) -> Result<$crate::world::World, serde_json::Error> {
                let mut json = value.serialize(serde_json::value::Serializer)?;
                let obj = json.as_object_mut().unwrap();
                let world = $crate::world::World::new();
                $(obj.insert(
                    stringify!($add).to_string(),
                    world.$add.serialize(serde_json::value::Serializer)?
                );)*
                $(obj.remove(stringify!($remove));)*
                $crate::world::World::deserialize(json)
            }
        }

    }
}

const CURRENT_VERSION: u8 = 1;

pub fn save_world(world: &World, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    file.write(&[CURRENT_VERSION])?;
    bincode::serialize_into(file, world)?;
    Ok(())
}

pub fn load_world(path: &str) -> Result<World, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buf: [u8; 1] = [0; 1];
    let version = match file.read(&mut buf) {
        Ok(1) => Ok(buf[0]),
        _ => Err("File must not be empty"),
    }?;
    match version {
        1 => bincode::deserialize_from(file).map_err(Into::into),
        //1 => bincode::deserialize_from::<_, v3::World>(file)?.try_into()?,
        _ => Err(Box::new(bincode::ErrorKind::Custom(
            "Incompatible version".to_owned(),
        )))
        .map_err(Into::into),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Enum)]
pub enum DebugLevel {
    Error,
    Warning,
    Information,
    All,
}
