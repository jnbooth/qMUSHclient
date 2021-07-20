use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, Read, Write};

use crate::world::*;

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

#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    SerialError(bincode::Error),
    NotSave,
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::FileError(value)
    }
}
impl From<bincode::Error> for Error {
    fn from(value: bincode::Error) -> Self {
        Self::SerialError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileError(e) => write!(f, "{}", e),
            Self::SerialError(e) => write!(f, "{}", e),
            Self::NotSave => write!(f, "not a savefile"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::FileError(e) => Some(e),
            Self::SerialError(e) => Some(e),
            Self::NotSave => None,
        }
    }
}

pub fn save_world(world: &World, path: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(&[CURRENT_VERSION])?;
    bincode::serialize_into(file, world)?;
    Ok(())
}

pub fn load_world(path: &str) -> Result<World, Error> {
    let mut file = File::open(path)?;
    let mut buf: [u8; 1] = [0; 1];
    let version = match file.read(&mut buf) {
        Ok(1) => Ok(buf[0]),
        _ => Err(Error::NotSave),
    }?;
    match version {
        1 => bincode::deserialize_from(file).map_err(Into::into),
        //1 => bincode::deserialize_from::<_, v3::World>(file)?.try_into()?,
        _ => Err(Error::NotSave),
    }
}

#[cfg(test)]
mod tests {
    use qt_core::Key;

    use super::*;
    use crate::script::{Alias, Timer, Trigger};

    #[test]
    pub fn test_world_roundtrip() {
        let mut world = World::new();
        world.proxy_type = Some(ProxyType::Socks4);
        world.connect_method = Some(AutoConnect::Diku);
        world.timers.push(Timer::default());
        world.triggers.push(Trigger::default());
        world.aliases.push(Alias::default());
        world.keypad_shortcuts.insert(Key::Key0, String::new());
        let to_file = bincode::serialize(&world).expect("error serializing world");
        let from_file = bincode::deserialize::<World>(&to_file).expect("error deserializing world");
        assert_eq!(from_file, world);
    }
}
