mod spawnset;
mod byte_reader;

use std::path::Path;
use spawnset::Spawnset;

fn main() {
	let spawnset = Spawnset::read_from_file(Path::new("/home/zee/storage1/SteamLibrary/steamapps/common/devildaggers/dd/survival"));
	spawnset.print();
}
