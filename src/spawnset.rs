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

    pub fn read_from_file(path: &std::path::Path) -> Spawnset {
        let contents = std::fs::read(path)
            .expect("Failed to read spawnset file");
        if contents.len() < 10480 {
        }

        let mut byte_reader = ByteReader::new(contents);
        let mut bytes_left = byte_reader.bytes_left();

        let header = Header::from_byte_reader(&mut byte_reader);
        println!("header bytes read: {}", bytes_left-byte_reader.bytes_left());
        bytes_left = byte_reader.bytes_left();

        let arena = Arena::from_byte_reader(&mut byte_reader);
        println!("arena bytes read: {}", bytes_left-byte_reader.bytes_left());
        bytes_left = byte_reader.bytes_left();

        let spawns_header = SpawnsHeader::from_byte_reader(&mut byte_reader);
        println!("spawns header bytes read: {}", bytes_left-byte_reader.bytes_left());
        bytes_left = byte_reader.bytes_left();

        let spawns_count = spawns_header.get_spawns_count() as usize;
        let mut spawns: Vec<Spawn> = Vec::with_capacity(spawns_count);
        for _i in 0..spawns_count {
            spawns.push(Spawn::from_byte_reader(&mut byte_reader))
        }
        println!("spawns bytes read: {}", bytes_left-byte_reader.bytes_left());
        bytes_left = byte_reader.bytes_left();

        //check world ver
        let spawn_version = header.get_spawn_version();
        let mut settings = None;
        if spawn_version > 4 {
            settings = Some(Settings::from_byte_reader(&mut byte_reader, spawn_version));
        }

        Spawnset::new(header, arena, spawns_header, spawns, settings)
    }

    pub fn print(self) {
        println!("Header:");
        println!("\tspawn_version: {}", self.header.get_spawn_version());
        println!("\tworld_version: {}", self.header.get_world_version());
        println!("\tshrink end radius: {}", self.header.get_shrink_end());
        println!("\tshrink start radius: {}", self.header.get_shrink_start());
        println!("\tshrink rate: {}", self.header.get_shrink_rate());
        println!("\tbrightness: {}", self.header.get_brightness());
        println!("\tgame mode: {}\n", self.header.get_game_mode());

        println!("Arena:");
        for y in 0..51 {
            for x in 0..51 {
                print!("{:.0} ", self.arena.get_at(x, y))
            }
            println!("")
        }
        println!("");

        println!("Spawns header:");
        println!("\tdevil dagger unlock time: {}", self.spawns_header.get_devil_dagger_unlock_time());
        println!("\tgolden dagger unlock time: {}", self.spawns_header.get_golden_dagger_unlock_time());
        println!("\tsilver dagger unlock time: {}", self.spawns_header.get_silver_dagger_unlock_time());
        println!("\tbronze dagger unlock time: {}", self.spawns_header.get_bronze_dagger_unlock_time());
        println!("\tspawns count: {}\n", self.spawns_header.get_spawns_count());

        println!("Spawns:");
        for spawn in &self.spawns {
            println!("\tenemy type: {}", spawn.get_enemy_type());
            println!("\tspawn delay: {:.1}\n", spawn.get_spawn_delay());
        }

        if self.header.get_spawn_version() > 4 {
            let settings = self.settings.unwrap();
            println!("Settings:");
            println!("\tInitial hand upgrade: {}", settings.get_initial_hand());
            println!("\tAdditional gems: {}", settings.get_additional_gems());
            if settings.get_time_start().is_some() {
                println!("\tget time start: {}", settings.get_time_start().unwrap());
            }
        }

        println!("done!")
    }
}