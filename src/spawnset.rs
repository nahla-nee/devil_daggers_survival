mod header;
mod arena;
mod spawns_header;
mod spawn;
mod settings;
mod enemy_type;
mod utils;

use std::path::Path;
use std::io::Cursor;

pub use header::Header;
pub use arena::Arena;
pub use spawn::Spawn;
pub use settings::Settings;
pub use enemy_type::EnemyType;

use crate::dd_error::DDError;

#[cfg_attr(feature = "json_coding", derive(Serialize, Deserialize))]
pub struct Spawnset {
    pub header: Header,
    pub arena: Arena,
    pub spawns: Vec<Spawn>,
    pub settings: Settings
}

impl Spawnset {
    const DEFAULT_SPAWN: &'static [u8] = include_bytes!("../assets/survival");

    pub fn new(header: Header, arena: Arena, spawns: Vec<Spawn>,
               settings: Settings) -> Spawnset {
        Spawnset {
            header,
            arena,
            spawns,
            settings
        }
    }

    pub fn default_spawn() -> Result<Spawnset, DDError> {
        Self::read_from_bytes(Self::DEFAULT_SPAWN)
    }

    pub fn read_from_file<P: AsRef<Path>>(path: &P) -> Result<Spawnset, DDError> {
        let contents = std::fs::read(path).or_else(|e| Err(DDError::IORead(e)))?;
        Self::read_from_bytes(&contents)
    }

    pub fn read_from_bytes(contents: &[u8]) -> Result<Spawnset, DDError> {
        let contents_size = contents.len();
        // the minimum size we need, this will have to be continously update as we get more info
        let mut needed_size = Header::size()+Arena::size();

        if contents_size < needed_size {
            return Err(DDError::NotEnoughDataRead);
        }

        let mut contents = Cursor::new(contents);

        // we know we can read at least the header and the arena
        let header = Header::from_reader(&mut contents);

        let arena = Arena::from_reader(&mut contents);

        // do we have enough data for the header?
        needed_size = needed_size+spawns_header::size(header.world_version);
        if contents_size < needed_size {
            return Err(DDError::NotEnoughDataRead);
        }

        let spawns_count = spawns_header::from_reader(header.world_version, &mut contents) as usize;

        // do we have enough data for the spawns and the settings?
        needed_size = needed_size+Spawn::size()*spawns_count+Settings::size(header.spawn_version);
        if contents_size < needed_size {
            return Err(DDError::NotEnoughDataRead);
        }

        let spawns: Vec<Spawn> = (0..spawns_count)
            .map(|_| Spawn::from_reader(&mut contents))
            .collect::<Vec<_>>();

        let settings = Settings::from_reader(header.spawn_version, &mut contents);

        Ok(Spawnset::new(header, arena, spawns, settings))
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: &P) -> Result<(), DDError> {
        // header: 36 bytes
        // arena: 51*51*4 bytes
        // spawns header: 40 or 36 bytes
        // each spawn: 28 bytes
        // settings: doesn't exist, or is 9 or 5 bytes
        let contents: Vec<u8> = vec![0u8; self.calculate_size()];
        let mut contents = Cursor::new(contents);

        self.header.to_writer(&mut contents);

        self.arena.to_writer(&mut contents);

        spawns_header::to_writer(self.spawns.len().try_into().unwrap(),
            self.header.world_version, &mut contents);

        for spawn in &self.spawns {
            spawn.to_writer(&mut contents);
        }

        self.settings.to_writer(self.header.spawn_version, &mut contents);

        std::fs::write(path, contents.get_ref())
            .or_else(|e| Err(DDError::IOWrite(e)))?;

        Ok(())
    }

    pub fn to_components(self) -> (Header, Arena, Vec<Spawn>, Settings) {
        (self.header, self.arena, self.spawns, self.settings)
    }

    pub fn calculate_size(&self) -> usize {
        Header::size()+Arena::size()+spawns_header::size(self.header.world_version)
        +self.spawns.len()*Spawn::size()+
        Settings::size(self.header.spawn_version)
    }

    pub fn print(&self) {
        self.print_header();
        println!();
        self.print_arena();
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

    pub fn print_spawns(&self) {
        println!("Spawns: {}", self.spawns.len());
        for spawn in &self.spawns {
            println!("\tenemy type: {}", spawn.enemy_type);
            println!("\tspawn delay: {:.1}", spawn.spawn_delay);
        }
    }

    pub fn print_settings(&self) {
        if self.header.spawn_version > 4 {
            println!("Settings:");
            println!("\tInitial hand upgrade: {}", self.settings.initial_hand);
            println!("\tAdditional gems: {}", self.settings.additional_gems);
            println!("\tget time start: {}", self.settings.time_start);
        }
    }
}