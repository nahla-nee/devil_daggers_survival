#[derive(Clone)]
#[cfg_attr(feature = "json_coding", derive(Serialize, Deserialize))]
pub enum EnemyType {
    SquidI,
    SquidII,
    Centipede,
    SpiderI,
    Leviathan,
    Gigapede,
    SquidIII,
    Thorn,
    SpiderII,
    Ghostpede,
    Empty = -1,
}

impl EnemyType {
    pub fn from_i32(val: i32) -> EnemyType {
        match val {
            0 => EnemyType::SquidI,
            1 => EnemyType::SquidII,
            2 => EnemyType::Centipede,
            3 => EnemyType::SpiderI,
            4 => EnemyType::Leviathan,
            5 => EnemyType::Gigapede,
            6 => EnemyType::SquidIII,
            7 => EnemyType::Thorn,
            8 => EnemyType::SpiderII,
            9 => EnemyType::Ghostpede,
            _ => EnemyType::Empty
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::SquidI => 0,
            Self::SquidII => 1,
            Self::Centipede => 2,
            Self::SpiderI => 3,
            Self::Leviathan => 4,
            Self::Gigapede => 5,
            Self::SquidIII => 6,
            Self::Thorn => 7,
            Self::SpiderII => 8,
            Self::Ghostpede => 9,
            Self::Empty => -1
        }
    }
}

impl std::fmt::Display for EnemyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EnemyType::SquidI => write!(f, "SquidI"),
            EnemyType::SquidII => write!(f, "SquidII"),
            EnemyType::Centipede => write!(f, "Centipede"),
            EnemyType::SpiderI => write!(f, "SpiderI"),
            EnemyType::Leviathan => write!(f, "Leviathan"),
            EnemyType::Gigapede => write!(f, "Gigapede"),
            EnemyType::SquidIII => write!(f, "SquidIII"),
            EnemyType::Thorn => write!(f, "Thorn"),
            EnemyType::SpiderII => write!(f, "SpiderII"),
            EnemyType::Ghostpede => write!(f, "Ghostpede"),
            EnemyType::Empty => write!(f, "Empty")
        }
    }
}

