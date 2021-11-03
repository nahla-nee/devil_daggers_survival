mod header;
mod arena;
mod spawns_header;
mod spawn;
mod settings;
mod enemy_type;

pub use header::Header;
pub use arena::Arena;
pub use spawns_header::SpawnsHeader;
pub use spawn::Spawn;
pub use settings::Settings;

use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;

pub struct Spawnset {
    header: Header,
    arena: Arena,
    spawns_header: SpawnsHeader,
    spawns: Vec<Spawn>,
    settings: Option<Settings>
}

impl Spawnset {
    pub fn new(header: Header, arena: Arena, spawns_header: SpawnsHeader, spawns: Vec<Spawn>,
               settings: Option<Settings>) -> Spawnset {
        Spawnset {
            header,
            arena,
            spawns_header,
            spawns,
            settings
        }
    }

    pub fn read_from_file(path: &std::path::Path) -> Result<Spawnset, DDError> {
        let contents = std::fs::read(path).or_else(|e| Err(DDError::IORead(e)))?;

        let mut byte_reader = ByteReader::new(contents);
        
        let header = Header::from_byte_reader(&mut byte_reader)?;
        
        let arena = Arena::from_byte_reader(&mut byte_reader)?;
        
        let spawns_header = SpawnsHeader::from_byte_reader(&mut byte_reader)?;
        
        let spawns_count = spawns_header.spawns_count as usize;
        let spawns: Vec<Spawn> = (0..spawns_count)
            .map(|_| Spawn::from_byte_reader(&mut byte_reader))
            .collect::<Result<Vec<_>,_>>()
            .or_else(|e| Err(e))?;

        //check world ver
        let spawn_version = header.spawn_version;
        let mut settings = None;
        if spawn_version > 4 {
            settings = Some(Settings::from_byte_reader(&mut byte_reader, spawn_version)?);
        }

        Ok(Spawnset::new(header, arena, spawns_header, spawns, settings))
    }

    pub fn print(&self) {
        println!("Header:");
        println!("\tspawn_version: {}", self.header.spawn_version);
        println!("\tworld_version: {}", self.header.world_version);
        println!("\tshrink end radius: {:.2}", self.header.shrink_end);
        println!("\tshrink start radius: {:.2}", self.header.shrink_start);
        println!("\tshrink rate: {}", self.header.shrink_rate);
        println!("\tbrightness: {}", self.header.brightness);
        println!("\tgame mode: {}\n", self.header.game_mode);

        println!("Arena:");
        for y in 0..51 {
            for x in 0..51 {
                print!("{:.0} ", self.arena.get_at(x, y))
            }
            println!("")
        }
        println!("");

        println!("Spawns header:");
        println!("\tdevil dagger unlock time: {}", self.spawns_header.devil_dagger_unlock_time);
        println!("\tgolden dagger unlock time: {}", self.spawns_header.golden_dagger_unlock_time);
        println!("\tsilver dagger unlock time: {}", self.spawns_header.silver_dagger_unlock_time);
        println!("\tbronze dagger unlock time: {}", self.spawns_header.bronze_dagger_unlock_time);
        println!("\tspawns count: {}\n", self.spawns_header.spawns_count);

        println!("Spawns:");
        for spawn in &self.spawns {
            println!("\tenemy type: {}", spawn.enemy_type);
            println!("\tspawn delay: {:.1}\n", spawn.spawn_delay);
        }

        if self.header.spawn_version > 4 {
            let settings = self.settings.as_ref().unwrap();
            println!("Settings:");
            println!("\tInitial hand upgrade: {}", settings.initial_hand);
            println!("\tAdditional gems: {}", settings.additional_gems);
            if settings.time_start.is_some() {
                println!("\tget time start: {}", settings.time_start.unwrap());
            }
        }
    }
}