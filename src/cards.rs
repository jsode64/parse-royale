use crate::util::is_kebab_case;

pub struct Card {
    /// The card's name.
    pub name: &'static str,

    /// The card's mastery badge name.
    pub badge_name: &'static str,

    /// The card's in-game ID.
    pub id: i64,
}

impl Card {
    /// Returns a card with the given name badge name and ID.
    fn new(name: &'static str, badge_name: &'static str, id: i64) -> Self {
        Self {
            name,
            badge_name,
            id,
        }
    }

    /// Matches the given card name to a card.
    ///
    /// Returns an error if the name is not kebab-case or if it isn't a card.
    pub fn from_name(name: &str) -> Result<Self, String> {
        // Make sure the name is kebab case before doing mass string comparisons.
        // This also makes the tool more user-friendly.
        if !is_kebab_case(name) {
            return Err(format!("The card name \"{name}\" is not kebab-case"));
        }

        Ok(match name {
            "electro-spirit" => Self::new("Electro Spirit", "MasteryElectroSpirit", 26000084),
            "log" => Self::new("The Log", "MasteryLog", 28000011),
            "archer" => Self::new("Archers", "MasteryArcher", 26000001),
            "royal-giant" => Self::new("Royal Giant", "MasteryRoyalGiant", 26000024),
            "skeletons" => Self::new("Skeletons", "MasterySkeletons", 26000010),
            "elite-barbarians" => Self::new("Elite Barbarians", "MasteryAngryBarbarians", 26000043),
            "earthquake" => Self::new("Earthquake", "MasteryEarthquake", 28000014),
            "dark-prince" => Self::new("Dark Prince", "MasteryDarkPrince", 26000027),
            "mega-minion" => Self::new("Mega Minion", "MasteryMegaMinion", 26000039),
            "goblin-cage" => Self::new("Goblin Cage", "MasteryGoblinCage", 27000012),
            "fireball" => Self::new("Fireball", "MasteryFireball", 28000000),
            "snowball" => Self::new("Giant Snowball", "MasterySnowball", 28000017),
            "mini-pekka" => Self::new("Mini P.E.K.K.A", "MasteryMiniPekka", 26000018),
            "ice-spirit" => Self::new("Ice Spirit", "MasteryIceSpirits", 26000030),
            "poison" => Self::new("Poison", "MasteryPoison", 28000009),
            "hog-rider" => Self::new("Hog Rider", "MasteryHogRider", 26000021),
            "valkyrie" => Self::new("Valkyrie", "MasteryValkyrie", 26000011),
            "bats" => Self::new("Bats", "MasteryBats", 26000049),
            "skeleton-king" => Self::new("Skeleton King", "MasterySkeletonKing", 26000069),
            "cannon-cart" => Self::new("Cannon Cart", "MasteryMovingCannon", 26000054),
            "bomber" => Self::new("Bomber", "MasteryBomber", 26000013),
            "barbarian-barrel" => Self::new("Barbarian Barrel", "MasteryBarbLog", 28000015),
            "ice-golemite" => Self::new("Ice Golem", "MasteryIceGolemite", 26000038),
            "graveyard" => Self::new("Graveyard", "MasteryGraveyard", 28000010),
            "royal-delivery" => Self::new("Royal Delivery", "MasteryRoyalDelivery", 28000018),
            "skeleton-army" => Self::new("Skeleton Army", "MasterySkeletonArmy", 26000012),
            "tornado" => Self::new("Tornado", "MasteryTornado", 28000012),
            "mega-knight" => Self::new("Mega Knight", "MasteryMegaKnight", 26000055),
            "electro-wizard" => Self::new("Electro Wizard", "MasteryElectroWizard", 26000042),
            "pekka" => Self::new("P.E.K.K.A", "MasteryPekka", 26000004),
            "heal" => Self::new("Heal Spirit", "MasteryHeal", 28000016),
            "mirror" => Self::new("Mirror", "MasteryMirror", 28000006),
            "firecracker" => Self::new("Firecracker", "MasteryFirecracker", 26000064),
            "fire-spirit" => Self::new("Fire Spirit", "MasteryFireSpirits", 26000031),
            "mother-witch" => Self::new("Mother Witch", "MasteryWitchMother", 26000083),
            "clone" => Self::new("Clone", "MasteryClone", 28000013),
            "witch" => Self::new("Witch", "MasteryWitch", 26000007),
            "night-witch" => Self::new("Night Witch", "MasteryDarkWitch", 26000048),
            "rage" => Self::new("Rage", "MasteryRage", 28000002),
            "miner" => Self::new("Miner", "MasteryMiner", 26000032),
            "wallbreakers" => Self::new("Wall Breakers", "MasteryWallbreakers", 26000058),
            "tombstone" => Self::new("Tombstone", "MasteryTombstone", 27000009),
            "bowler" => Self::new("Bowler", "MasteryBowler", 26000034),
            "baby-dragon" => Self::new("Baby Dragon", "MasteryBabyDragon", 26000015),
            "arrows" => Self::new("Arrows", "MasteryArrows", 28000001),
            "elixir-golem" => Self::new("Elixir Golem", "MasteryElixirGolem", 26000067),
            "archer-queen" => Self::new("Archer Queen", "MasteryArcherQueen", 26000072),
            "golem" => Self::new("Golem", "MasteryGolem", 26000009),
            "royal-hogs" => Self::new("Royal Hogs", "MasteryRoyalHogs", 26000059),
            "electro-dragon" => Self::new("Electro Dragon", "MasteryElectroDragon", 26000063),
            "zap" => Self::new("Zap", "MasteryZap", 28000008),
            "rocket" => Self::new("Rocket", "MasteryRocket", 28000003),
            "zap-machine" => Self::new("Zap Machine", "MasteryZapMachine", 26000033),
            "battle-healer" => Self::new("Battle Healer", "MasteryBattleHealer", 26000068),
            "lightning" => Self::new("Lightning", "MasteryLightning", 28000007),
            "skeleton-balloon" => Self::new("Skeleton Barrel", "MasterySkeletonBalloon", 26000056),
            "freeze" => Self::new("Freeze", "MasteryFreeze", 28000005),
            "goblin-gang" => Self::new("Goblin Gang", "MasteryGoblinGang", 26000041),
            "goblin-barrel" => Self::new("Goblin Barrel", "MasteryGoblinBarrel", 28000004),
            "inferno-tower" => Self::new("Inferno Tower", "MasteryInfernoTower", 27000003),
            "princess" => Self::new("Princess", "MasteryPrincess", 26000026),
            "tesla" => Self::new("Tesla", "MasteryTesla", 27000006),
            "inferno-dragon" => Self::new("Inferno Dragon", "MasteryInfernoDragon", 26000037),
            "goblin-giant" => Self::new("Goblin Giant", "MasteryGoblinGiant", 26000060),
            "minion-horde" => Self::new("Minion Horde", "MasteryMinionHorde", 26000022),
            "bomb-tower" => Self::new("Bomb Tower", "MasteryBombTower", 27000004),
            "dart-barrell" => Self::new("Dart Goblin", "MasteryDartBarrell", 26000040),
            "goblin-drill" => Self::new("Goblin Drill", "MasteryGoblinDrill", 27000013),
            "blowdart-goblin" => Self::new("Dart Goblin", "MasteryBlowdartGoblin", 26000040),
            "giant-skeleton" => Self::new("Giant Skeleton", "MasteryGiantSkeleton", 26000020),
            "barbarians" => Self::new("Barbarians", "MasteryBarbarians", 26000008),
            "elixir-collector" => {
                Self::new("Elixir Collector", "MasteryElixir Collector", 27000007)
            }
            "spear-goblins" => Self::new("Spear Goblins", "MasterySpearGoblins", 26000019),
            "mortar" => Self::new("Mortar", "MasteryMortar", 27000002),
            "fisherman" => Self::new("Fisherman", "MasteryFisherman", 26000061),
            "wizard" => Self::new("Wizard", "MasteryWizard", 26000017),
            "giant" => Self::new("Giant", "MasteryGiant", 26000003),
            "goblins" => Self::new("Goblins", "MasteryGoblins", 26000002),
            "golden-knight" => Self::new("Golden Knight", "MasteryGoldenKnight", 26000074),
            "barbarian-hut" => Self::new("Barbarian Hut", "MasteryBarbarianHut", 27000005),
            "xbow" => Self::new("X-Bow", "MasteryXbow", 27000008),
            "zappies" => Self::new("Zappies", "MasteryMiniSparkys", 26000052),
            "hunter" => Self::new("Hunter", "MasteryHunter", 26000044),
            "musketeer" => Self::new("Musketeer", "MasteryMusketeer", 26000014),
            "lava-hound" => Self::new("Lava Hound", "MasteryLavaHound", 26000029),
            "cannon" => Self::new("Cannon", "MasteryCannon", 27000000),
            "electro-giant" => Self::new("Electro Giant", "MasteryElectroGiant", 26000085),
            "furnace" => Self::new("Furnace", "MasteryFirespiritHut", 27000010),
            "battle-ram" => Self::new("Battle Ram", "MasteryBattleRam", 26000036),
            "royal-recruits" => Self::new("Royal Recruits", "MasteryRoyalRecruits", 26000047),
            "banndit" => Self::new("Bandit", "MasteryAssassin", 26000046),
            "magic-archer" => Self::new("Magic Archer", "MasteryEliteArcher", 26000062),
            "prince" => Self::new("Prince", "MasteryPrince", 26000016),
            "lumberjack" => Self::new("Lumberjack", "MasteryRageBarbarian", 26000035),
            "royal-ghost" => Self::new("Royal Ghost", "MasteryGhost", 26000050),
            "rascals" => Self::new("Rascals", "MasteryRascals", 26000053),
            "guards" => Self::new("Guards", "MasterySkeletonWarriors", 26000025),
            "skeleton-dragons" => Self::new("Skeleton Dragons", "MasterySkeletonDragons", 26000080),
            "executioner" => Self::new("Executioner", "MasteryAxeMan", 26000045),
            "ice-wizard" => Self::new("Ice Wizard", "MasteryIceWizard", 26000023),
            "goblin-hut" => Self::new("Goblin Hut", "MasteryGoblinHut", 27000001),
            "three-musketeers" => Self::new("Three Musketeers", "MasteryThreeMusketeers", 26000028),
            "ram-rider" => Self::new("Ram Rider", "MasteryRamRider", 26000051),
            "mighty-miner" => Self::new("Mighty Miner", "MasteryMightyMiner", 26000065),
            "phoenix" => Self::new("Phoenix", "MasteryPhoenix", 26000087),
            "monk" => Self::new("Monk", "MasteryMonk", 26000077),
            "little-prince" => Self::new("Little Prince", "MasteryLittlePrince", 26000093),
            "void" => Self::new("Void", "MasteryDarkMagic", 28000023),
            "goblinstein" => Self::new("Goblinstein", "MasteryGoblinstein", 26000099),
            "suspicious-bush" => Self::new("Suspicious Bush", "MasterySuspiciousBush", 26000097),
            "goblin-demolisher" => {
                Self::new("Goblin Demolisher", "MasteryGoblinDemolisher", 26000095)
            }
            "berserker" => Self::new("Berserker", "MasteryBerserker", 26000102),
            "boss-bandit" => Self::new("Boss Bandit", "MasteryBossBandit", 26000103),
            "goblin-machine" => Self::new("Goblin Machine", "MasteryGoblinMachine", 26000096),
            "goblin-curse" => Self::new("Goblin Curse", "MasteryGoblinCurse", 28000024),
            "spirit-empress" => Self::new("Spirit Empress", "MasteryMergeMaiden", 28000025),
            "rune-giant" => Self::new("Rune Giant", "MasteryGiantBuffer", 26000101),
            "vines" => Self::new("Vines", "MasteryVines", 28000026),
            "knight" => Self::new("Knight", "MasteryKnight", 26000000),
            "minions" => Self::new("Minions", "MasteryMinions", 26000005),
            "balloon" => Self::new("Balloon", "MasteryBalloon", 26000006),
            "sparky" => Self::new("Sparky", "MasteryZapMachine", 26000033),

            _ => return Err(format!("Unknown card name: \"{name}\"")),
        })
    }
}
