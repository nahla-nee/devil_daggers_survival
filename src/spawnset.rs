mod header;
mod arena;
mod spawns_header;
mod spawn;
mod settings;
mod enemy_type;

use std::path::Path;

pub use header::Header;
pub use arena::Arena;
pub use spawns_header::SpawnsHeader;
pub use spawn::Spawn;
pub use settings::Settings;

use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;

pub struct Spawnset {
    pub header: Header,
    pub arena: Arena,
    pub spawns_header: SpawnsHeader,
    pub spawns: Vec<Spawn>,
    pub settings: Option<Settings>
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

    pub fn read_from_file<P: AsRef<Path>>(path: &P) -> Result<Spawnset, DDError> {
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
        let settings = Settings::from_byte_reader(&mut byte_reader, spawn_version)?;

        Ok(Spawnset::new(header, arena, spawns_header, spawns, settings))
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: &P) -> Result<(), DDError> {
        //header: 36 bytes
        //arena: 51*51*4 bytes
        //spawns header: 40 bytes
        //each spawn: 28 bytes
        //settings: doesn't exist, is 9 or 5 bytes, allocate 9 and decide later
        let mut data: Vec<u8> = vec![0u8; self.calculate_size()];

        let mut data_written = 0;

        self.header.to_byte_slice(&mut data[0..data_written+Header::size()])?;
        data_written = Header::size();

        self.arena.to_byte_slice(&mut data[data_written..data_written+Arena::size()])?;
        data_written = data_written+Arena::size();

        self.spawns_header.to_byte_slice(&mut data[data_written..data_written+SpawnsHeader::size()])?;
        data_written = data_written+SpawnsHeader::size();

        for spawn in &self.spawns {
            spawn.to_byte_slice(&mut data[data_written..data_written+Spawn::size()])?;
            data_written = data_written+Spawn::size();
        }

        let spawn_version = self.header.spawn_version;
        let settings_size = Settings::size_from_spawn_ver(spawn_version);
        if self.settings.is_some(){
            self.settings.as_ref().unwrap().to_byte_slice(&mut data[data_written..data_written+settings_size], spawn_version)?;
        }

        std::fs::write(path, data.as_slice()).or_else(|e| Err(DDError::IOWrite(e)))?;

        Ok(())
    }

    pub fn calculate_size(&self) -> usize {
        Header::size()+Arena::size()+SpawnsHeader::size()+self.spawns.len()*Spawn::size()
        +Settings::size_from_spawn_ver(self.header.spawn_version)
    }

    pub fn print(&self) {
        self.print_header();
        println!();
        self.print_arena();
        println!();
        self.print_spawns_header();
        println!();
        self.print_spawns();
        println!();
        self.print_settings();
    }

    pub fn print_header(&self) {
        println!("Header:");
        println!("\tspawn_version: {}", self.header.spawn_version);
        println!("\tworld_version: {}", self.header.world_version);
        println!("\tshrink end radius: {:.2}", self.header.shrink_end);
        println!("\tshrink start radius: {:.2}", self.header.shrink_start);
        println!("\tshrink rate: {}", self.header.shrink_rate);
        println!("\tbrightness: {}", self.header.brightness);
        println!("\tgame mode: {}", self.header.game_mode);
    }

    pub fn print_arena(&self) {
        println!("Arena:");
        for y in 0..51 {
            for x in 0..51 {
                print!("{:.0} ", self.arena.get_at(x, y))
            }
            println!("")
        }
    }

    pub fn print_spawns_header(&self) {
        println!("Spawns header:");
        println!("\tdevil dagger unlock time: {}", self.spawns_header.devil_dagger_unlock_time);
        println!("\tgolden dagger unlock time: {}", self.spawns_header.golden_dagger_unlock_time);
        println!("\tsilver dagger unlock time: {}", self.spawns_header.silver_dagger_unlock_time);
        println!("\tbronze dagger unlock time: {}", self.spawns_header.bronze_dagger_unlock_time);
        println!("\tspawns count: {}", self.spawns_header.spawns_count);
    }

    pub fn print_spawns(&self) {
        println!("Spawns:");
        for spawn in &self.spawns {
            println!("\tenemy type: {}", spawn.enemy_type);
            println!("\tspawn delay: {:.1}", spawn.spawn_delay);
        }
    }

    pub fn print_settings(&self) {
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