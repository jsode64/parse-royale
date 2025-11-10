use crate::util::is_kebab_case;

pub struct Card {
    /// The card's name.
    pub name: &'static str,

    /// The card's mastery badge name.
    pub badge_name: &'static str,

    /// The card's in-game ID.
    pub id: i64,
}

macro_rules! def_cards {
    ($($name:ident: $card_name:expr, $badge_name:expr, $id: expr);* ;) => {
        $(const $name: Card = Card::new($card_name, $badge_name, $id);)*
    };
}

impl Card {
    /// Returns a card with the given name badge name and ID.
    const fn new(name: &'static str, badge_name: &'static str, id: i64) -> Self {
        Self {
            name,
            badge_name,
            id,
        }
    }

    /// Matches the given card name to a card.
    ///
    /// Returns an error if the name is not kebab-case or if it isn't a card.
    pub fn from_name(name: &str) -> Result<&'static Self, String> {
        // Make sure the name is kebab case before doing mass string comparisons.
        if !is_kebab_case(name) {
            return Err(format!("The card name \"{name}\" is not kebab-case"));
        }

        Ok(match name {
            "knight" => &Self::KNIGHT,
            "archers" => &Self::ARCHERS,
            "goblins" => &Self::GOBLINS,
            "giant" => &Self::GIANT,
            "pekka" => &Self::PEKKA,
            "minions" => &Self::MINIONS,
            "balloon" => &Self::BALLOON,
            "witch" => &Self::WITCH,
            "barbarians" => &Self::BARBARIANS,
            "golem" => &Self::GOLEM,
            "skeletons" => &Self::SKELETONS,
            "valkyrie" => &Self::VALKYRIE,
            "skeleton-army" => &Self::SKELETON_ARMY,
            "bomber" => &Self::BOMBER,
            "musketeer" => &Self::MUSKETEER,
            "baby-dragon" => &Self::BABY_DRAGON,
            "prince" => &Self::PRINCE,
            "wizard" => &Self::WIZARD,
            "mini-pekka" => &Self::MINI_PEKKA,
            "spear-goblins" => &Self::SPEAR_GOBLINS,
            "giant-skeleton" => &Self::GIANT_SKELETON,
            "hog-rider" => &Self::HOG_RIDER,
            "minion-horde" => &Self::MINION_HORDE,
            "ice-wizard" => &Self::ICE_WIZARD,
            "royal-giant" => &Self::ROYAL_GIANT,
            "guards" => &Self::GUARDS,
            "princess" => &Self::PRINCESS,
            "dark-prince" => &Self::DARK_PRINCE,
            "three-musketeers" => &Self::THREE_MUSKETEERS,
            "lava-hound" => &Self::LAVA_HOUND,
            "ice-spirit" => &Self::ICE_SPIRIT,
            "fire-spirit" => &Self::FIRE_SPIRIT,
            "miner" => &Self::MINER,
            "sparky" => &Self::SPARKY,
            "bowler" => &Self::BOWLER,
            "lumberjack" => &Self::LUMBERJACK,
            "battle-ram" => &Self::BATTLE_RAM,
            "inferno-dragon" => &Self::INFERNO_DRAGON,
            "ice-golem" => &Self::ICE_GOLEM,
            "mega-minion" => &Self::MEGA_MINION,
            "dart-goblin" => &Self::DART_GOBLIN,
            "goblin-gang" => &Self::GOBLIN_GANG,
            "electro-wizard" => &Self::ELECTRO_WIZARD,
            "elite-barbarians" => &Self::ELITE_BARBARIANS,
            "hunter" => &Self::HUNTER,
            "executioner" => &Self::EXECUTIONER,
            "bandit" => &Self::BANDIT,
            "royal-recruits" => &Self::ROYAL_RECRUITS,
            "night-witch" => &Self::NIGHT_WITCH,
            "bats" => &Self::BATS,
            "royal-ghost" => &Self::ROYAL_GHOST,
            "ram-rider" => &Self::RAM_RIDER,
            "zappies" => &Self::ZAPPIES,
            "rascals" => &Self::RASCALS,
            "cannon-cart" => &Self::CANNON_CART,
            "mega-knight" => &Self::MEGA_KNIGHT,
            "skeleton-barrel" => &Self::SKELETON_BARREL,
            "flying-machine" => &Self::FLYING_MACHINE,
            "wall-breakers" => &Self::WALL_BREAKERS,
            "royal-hogs" => &Self::ROYAL_HOGS,
            "goblin-giant" => &Self::GOBLIN_GIANT,
            "fisherman" => &Self::FISHERMAN,
            "magic-archer" => &Self::MAGIC_ARCHER,
            "electro-dragon" => &Self::ELECTRO_DRAGON,
            "firecracker" => &Self::FIRECRACKER,
            "mighty-miner" => &Self::MIGHTY_MINER,
            "elixir-golem" => &Self::ELIXIR_GOLEM,
            "battle-healer" => &Self::BATTLE_HEALER,
            "skeleton-king" => &Self::SKELETON_KING,
            "archer-queen" => &Self::ARCHER_QUEEN,
            "golden-knight" => &Self::GOLDEN_KNIGHT,
            "monk" => &Self::MONK,
            "skeleton-dragons" => &Self::SKELETON_DRAGONS,
            "mother-witch" => &Self::MOTHER_WITCH,
            "electro-spirit" => &Self::ELECTRO_SPIRIT,
            "electro-giant" => &Self::ELECTRO_GIANT,
            "phoenix" => &Self::PHOENIX,
            "little-prince" => &Self::LITTLE_PRINCE,
            "goblin-demolisher" => &Self::GOBLIN_DEMOLISHER,
            "goblin-machine" => &Self::GOBLIN_MACHINE,
            "suspicious-bush" => &Self::SUSPICIOUS_BUSH,
            "goblinstein" => &Self::GOBLINSTEIN,
            "rune-giant" => &Self::RUNE_GIANT,
            "berserker" => &Self::BERSERKER,
            "boss-bandit" => &Self::BOSS_BANDIT,
            "cannon" => &Self::CANNON,
            "goblin-hut" => &Self::GOBLIN_HUT,
            "mortar" => &Self::MORTAR,
            "inferno-tower" => &Self::INFERNO_TOWER,
            "bomb-tower" => &Self::BOMB_TOWER,
            "barbarian-hut" => &Self::BARBARIAN_HUT,
            "tesla" => &Self::TESLA,
            "elixir-collector" => &Self::ELIXIR_COLLECTOR,
            "x-bow" => &Self::X_BOW,
            "tombstone" => &Self::TOMBSTONE,
            "furnace" => &Self::FURNACE,
            "goblin-cage" => &Self::GOBLIN_CAGE,
            "goblin-drill" => &Self::GOBLIN_DRILL,
            "fireball" => &Self::FIREBALL,
            "arrows" => &Self::ARROWS,
            "rage" => &Self::RAGE,
            "rocket" => &Self::ROCKET,
            "goblin-barrel" => &Self::GOBLIN_BARREL,
            "freeze" => &Self::FREEZE,
            "mirror" => &Self::MIRROR,
            "lightning" => &Self::LIGHTNING,
            "zap" => &Self::ZAP,
            "poison" => &Self::POISON,
            "graveyard" => &Self::GRAVEYARD,
            "the-log" => &Self::THE_LOG,
            "tornado" => &Self::TORNADO,
            "clone" => &Self::CLONE,
            "earthquake" => &Self::EARTHQUAKE,
            "barbarian-barrel" => &Self::BARBARIAN_BARREL,
            "heal-spirit" => &Self::HEAL_SPIRIT,
            "giant-snowball" => &Self::GIANT_SNOWBALL,
            "royal-delivery" => &Self::ROYAL_DELIVERY,
            "void" => &Self::VOID,
            "goblin-curse" => &Self::GOBLIN_CURSE,
            "spirit-empress" => &Self::SPIRIT_EMPRESS,
            "vines" => &Self::VINES,

            _ => return Err(format!("Unknown card name: \"{name}\"")),
        })
    }

    def_cards!(
        KNIGHT: "Knight", "MasteryKnight", 26000000;
        ARCHERS: "Archers", "MasteryArchers", 26000001;
        GOBLINS: "Goblins", "MasteryGoblins", 26000002;
        GIANT: "Giant", "MasteryGiant", 26000003;
        PEKKA: "P.E.K.K.A", "MasteryPekka", 26000004;
        MINIONS: "Minions", "MasteryMinions", 26000005;
        BALLOON: "Balloon", "MasteryBalloon", 26000006;
        WITCH: "Witch", "MasteryWitch", 26000007;
        BARBARIANS: "Barbarians", "MasteryBarbarians", 26000008;
        GOLEM: "Golem", "MasteryGolem", 26000009;
        SKELETONS: "Skeletons", "MasterySkeletons", 26000010;
        VALKYRIE: "Valkyrie", "MasteryValkyrie", 26000011;
        SKELETON_ARMY: "Skeleton Army", "MasterySkeletonArmy", 26000012;
        BOMBER: "Bomber", "MasteryBomber", 26000013;
        MUSKETEER: "Musketeer", "MasteryMusketeer", 26000014;
        BABY_DRAGON: "Baby Dragon", "MasteryBabyDragon", 26000015;
        PRINCE: "Prince", "MasteryPrince", 26000016;
        WIZARD: "Wizard", "MasteryWizard", 26000017;
        MINI_PEKKA: "Mini P.E.K.K.A", "MasteryMiniPekka", 26000018;
        SPEAR_GOBLINS: "Spear Goblins", "MasterySpearGoblins", 26000019;
        GIANT_SKELETON: "Giant Skeleton", "MasteryGiantSkeleton", 26000020;
        HOG_RIDER: "Hog Rider", "MasteryHogRider", 26000021;
        MINION_HORDE: "Minion Horde", "MasteryMinionHorde", 26000022;
        ICE_WIZARD: "Ice Wizard", "MasteryIceWizard", 26000023;
        ROYAL_GIANT: "Royal Giant", "MasteryRoyalGiant", 26000024;
        GUARDS: "Guards", "MasterySkeletonWarriors", 26000025;
        PRINCESS: "Princess", "MasteryPrincess", 26000026;
        DARK_PRINCE: "Dark Prince", "MasteryDarkPrince", 26000027;
        THREE_MUSKETEERS: "Three Musketeers", "MasteryThreeMusketeers", 26000028;
        LAVA_HOUND: "Lava Hound", "MasteryLavaHound", 26000029;
        ICE_SPIRIT: "Ice Spirit", "MasteryIceSpirits", 26000030;
        FIRE_SPIRIT: "Fire Spirit", "MasteryFireSpirit", 26000031;
        MINER: "Miner", "MasteryMiner", 26000032;
        SPARKY: "Sparky", "MasteryZapMachine", 26000033;
        BOWLER: "Bowler", "MasteryBowler", 26000034;
        LUMBERJACK: "Lumberjack", "MasteryRageBarbarian", 26000035;
        BATTLE_RAM: "Battle Ram", "MasteryBattleRam", 26000036;
        INFERNO_DRAGON: "Inferno Dragon", "MasteryInfernoDragon", 26000037;
        ICE_GOLEM: "Ice Golem", "MasteryIceGolemite", 26000038;
        MEGA_MINION: "Mega Minion", "MasteryMegaMinion", 26000039;
        DART_GOBLIN: "Dart Goblin", "MasteryBlowdartGoblin", 26000040;
        GOBLIN_GANG: "Goblin Gang", "MasteryGoblinGang", 26000041;
        ELECTRO_WIZARD: "Electro Wizard", "MasteryElectroWizard", 26000042;
        ELITE_BARBARIANS: "Elite Barbarians", "MasteryAngryBarbarians", 26000043;
        HUNTER: "Hunter", "MasteryHunter", 26000044;
        EXECUTIONER: "Executioner", "MasteryAxeMan", 26000045;
        BANDIT: "Bandit", "MasteryAssassin", 26000046;
        ROYAL_RECRUITS: "Royal Recruits", "MasteryRoyalRecruits", 26000047;
        NIGHT_WITCH: "Night Witch", "MasteryDarkWitch", 26000048;
        BATS: "Bats", "MasteryBats", 26000049;
        ROYAL_GHOST: "Royal Ghost", "MasteryGhost", 26000050;
        RAM_RIDER: "Ram Rider", "MasteryRamRider", 26000051;
        ZAPPIES: "Zappies", "MasteryMiniSparkys", 26000052;
        RASCALS: "Rascals", "MasteryRascals", 26000053;
        CANNON_CART: "Cannon Cart", "MasteryMovingCannon", 26000054;
        MEGA_KNIGHT: "Mega Knight", "MasteryMegaKnight", 26000055;
        SKELETON_BARREL: "Skeleton Barrel", "MasterySkeletonBalloon", 26000056;
        FLYING_MACHINE: "Flying Machine", "MasteryDartBarrell", 26000057;
        WALL_BREAKERS: "Wall Breakers", "MasteryWallBreakers", 26000058;
        ROYAL_HOGS: "Royal Hogs", "MasteryRoyalHogs", 26000059;
        GOBLIN_GIANT: "Goblin Giant", "MasteryGoblinGiant", 26000060;
        FISHERMAN: "Fisherman", "MasteryFisherman", 26000061;
        MAGIC_ARCHER: "Magic Archer", "MasteryEliteArcher", 26000062;
        ELECTRO_DRAGON: "Electro Dragon", "MasteryElectroDragon", 26000063;
        FIRECRACKER: "Firecracker", "MasteryFirecracker", 26000064;
        MIGHTY_MINER: "Mighty Miner", "MasteryMightyMiner", 26000065;
        ELIXIR_GOLEM: "Elixir Golem", "MasteryElixirGolem", 26000067;
        BATTLE_HEALER: "Battle Healer", "MasteryBattleHealer", 26000068;
        SKELETON_KING: "Skeleton King", "MasterySkeletonKing", 26000069;
        ARCHER_QUEEN: "Archer Queen", "MasteryArcherQueen", 26000072;
        GOLDEN_KNIGHT: "Golden Knight", "MasteryGoldenKnight", 26000074;
        MONK: "Monk", "MasteryMonk", 26000077;
        SKELETON_DRAGONS: "Skeleton Dragons", "MasterySkeletonDragons", 26000080;
        MOTHER_WITCH: "Mother Witch", "MasteryWitchMother", 26000083;
        ELECTRO_SPIRIT: "Electro Spirit", "MasteryElectroSpirit", 26000084;
        ELECTRO_GIANT: "Electro Giant", "MasteryElectroGiant", 26000085;
        PHOENIX: "Phoenix", "MasteryPhoenix", 26000087;
        LITTLE_PRINCE: "Little Prince", "MasteryLittlePrince", 26000093;
        GOBLIN_DEMOLISHER: "Goblin Demolisher", "MasteryGoblinDemolisher", 26000095;
        GOBLIN_MACHINE: "Goblin Machine", "MasteryGoblinMachine", 26000096;
        SUSPICIOUS_BUSH: "Suspicious Bush", "MasterySuspiciousBush", 26000097;
        GOBLINSTEIN: "Goblinstein", "MasteryGoblinstein", 26000099;
        RUNE_GIANT: "Rune Giant", "MasteryGiantBuffer", 26000101;
        BERSERKER: "Berserker", "MasteryBerserker", 26000102;
        BOSS_BANDIT: "Boss Bandit", "MasteryBossBandit", 26000103;
        CANNON: "Cannon", "MasteryCannon", 27000000;
        GOBLIN_HUT: "Goblin Hut", "MasteryGoblinHut", 27000001;
        MORTAR: "Mortar", "MasteryMortar", 27000002;
        INFERNO_TOWER: "Inferno Tower", "MasteryInfernoTower", 27000003;
        BOMB_TOWER: "Bomb Tower", "MasteryBombTower", 27000004;
        BARBARIAN_HUT: "Barbarian Hut", "MasteryBarbarianHut", 27000005;
        TESLA: "Tesla", "MasteryTesla", 27000006;
        ELIXIR_COLLECTOR: "Elixir Collector", "MasteryElixir Collector", 27000007;
        X_BOW: "X-Bow", "MasteryXBow", 27000008;
        TOMBSTONE: "Tombstone", "MasteryTombstone", 27000009;
        FURNACE: "Furnace", "MasteryFirespiritHut", 27000010;
        GOBLIN_CAGE: "Goblin Cage", "MasteryGoblinCage", 27000012;
        GOBLIN_DRILL: "Goblin Drill", "MasteryGoblinDrill", 27000013;
        FIREBALL: "Fireball", "MasteryFireball", 28000000;
        ARROWS: "Arrows", "MasteryArrows", 28000001;
        RAGE: "Rage", "MasteryRage", 28000002;
        ROCKET: "Rocket", "MasteryRocket", 28000003;
        GOBLIN_BARREL: "Goblin Barrel", "MasteryGoblinBarrel", 28000004;
        FREEZE: "Freeze", "MasteryFreeze", 28000005;
        MIRROR: "Mirror", "MasteryMirror", 28000006;
        LIGHTNING: "Lightning", "MasteryLightning", 28000007;
        ZAP: "Zap", "MasteryZap", 28000008;
        POISON: "Poison", "MasteryPoison", 28000009;
        GRAVEYARD: "Graveyard", "MasteryGraveyard", 28000010;
        THE_LOG: "The Log", "MasteryTheLog", 28000011;
        TORNADO: "Tornado", "MasteryTornado", 28000012;
        CLONE: "Clone", "MasteryClone", 28000013;
        EARTHQUAKE: "Earthquake", "MasteryEarthquake", 28000014;
        BARBARIAN_BARREL: "Barbarian Barrel", "MasteryBarbLog", 28000015;
        HEAL_SPIRIT: "Heal Spirit", "MasteryHeal", 28000016;
        GIANT_SNOWBALL: "Giant Snowball", "MasterySnowball", 28000017;
        ROYAL_DELIVERY: "Royal Delivery", "MasteryRoyalDelivery", 28000018;
        VOID: "Void", "MasteryDarkMagic", 28000023;
        GOBLIN_CURSE: "Goblin Curse", "MasteryGoblinCurse", 28000024;
        SPIRIT_EMPRESS: "Spirit Empress", "MasteryMergeMaiden", 28000025;
        VINES: "Vines", "MasteryVines", 28000026;
    );
}
