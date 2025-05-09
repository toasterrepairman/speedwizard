use std::collections::HashMap;
use lazy_static::lazy_static;

pub const CSV_DATA: &str = r#"system,object_class,name,atmospheric_pressure
Stanton,Planet,Hurston,0.89
Stanton,Moon,Arial,0.77
Stanton,Moon,Aberdeen,1.04
Stanton,Moon,Magda,0.33
Stanton,Moon,Ita,0.15
Stanton,Planet,Crusader,1.0
Stanton,Moon,Cellin,0.19
Stanton,Moon,Daymar,0.6
Stanton,Moon,Yela,0.19
Stanton,Planet,ArcCorp,0.84
Stanton,Moon,Lyria,0.09
Stanton,Moon,Wala,0.1
Stanton,Planet,microTech,1.1
Stanton,Moon,Calliope,0.8
Stanton,Moon,Clio,0.62
Stanton,Moon,Euterpe,0.79
Pyro,Planet,Pyro I,2.3824
Pyro,Planet,Monox,0.908
Pyro,Planet,Bloom,0.9889
Pyro,Planet,Pyro V,2.9854
Pyro,Moon,Ignis,0.077
Pyro,Moon,Vatra,1.207
Pyro,Moon,Adir,0.0493
Pyro,Moon,Fairo,1.7705
Pyro,Moon,Fuego,0.5606
Pyro,Moon,Vuur,1.2544
Pyro,Protoplanet,Pyro IV,0.4875
Pyro,Planet,Terminus,1.128
"#;

#[derive(Debug)]
pub struct PlanetInfo {
    pub description: String,
    pub difficulty: String,  // Emoji string
}

lazy_static! {
    pub static ref PLANET_INFO: HashMap<String, PlanetInfo> = {
        let mut m = HashMap::new();

        // Stanton System
        m.insert("Hurston".to_string(), PlanetInfo {
            description: "Hurston is a hot, arid planet that orbits closest to the sun in the Stanton system. Discovered in 2851, it was sold by the United Empire of Earth government to the company Hurston Dynamics in 2865. Unrestricted industry and mining have heavily polluted the atmosphere of the planet.".to_string(),
            difficulty: "⭐⭐".to_string(),
        });

        m.insert("Crusader".to_string(), PlanetInfo {
            description: "Crusader is a low mass gas giant in the Stanton system. Rare among gas giants, Crusader's upper atmosphere is breathable by Humans and supports an active indigenous biosphere. The company settled Crusader via a complex network of floating platforms.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("ArcCorp".to_string(), PlanetInfo {
            description: "ArcCorp is the third planet from the sun of the Stanton system. In just 80 years all of the terrain of the planet has been sculpted, zoned, and built upon, leaving very little left for nature. It consists of layer after layer of factory; so many that today building additional structures is actually impossible.".to_string(),
            difficulty: "⭐".to_string(),
        });

        m.insert("microTech".to_string(), PlanetInfo {
            description: "microTech is the fourth and farthest planet from the sun in the Stanton system. Located at the outer edge of the system's habitable zone, microTech is mostly ice, with some cold-to-temperate areas of tundra. Its capital, New Babbage, is home to many corporations and startups.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        // Add moons too
        m.insert("Daymar".to_string(), PlanetInfo {
            description: "Daymar is Crusader's second closest natural satellite. Underground race Daymar Rally is held on Daymar every Standard Earth Year. In 2862, the Javelin-class destroyer UEES Flyssa crashed on Daymar after an illegal mining spacecraft it was pursuing unexpectedly exploded.".to_string(),
            difficulty: "⭐⭐".to_string(),
        });

        m.insert("Cellin".to_string(), PlanetInfo {
            description: "Cellin is a volcanically active moon thronged with geysers that orbits Crusader, and is the planet's closest natural satellite. Its tenuous atmosphere contains trace amounts of oxygen, but not enough to support respiration.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Yela".to_string(), PlanetInfo {
            description: "Yela is the outermost natural satellite of Crusader, and is orbited by a ring of asteroids. A thin crust of water-ice on the moon's surface represents the cool demeanor of Yela, the oldest sibling in the Human children's story A Gift for Baba.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m
    };
}
