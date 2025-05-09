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

        m.insert("Arial".to_string(), PlanetInfo {
            description: "Arial is the first moon of Hurston, and is named for Arial Hurston, the creator of the Life/Labor-style contract utilized by Hurston Dynamics for their employees. It has a thin nitrogen-methane atmosphere and a rocky, dry landscape.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Aberdeen".to_string(), PlanetInfo {
            description: "Aberdeen, named for the designer of Hurston Dynamics' first antimatter warhead, is the second moon of Hurston. It has a thick, hazy atmosphere composed of sulfurous gas that creates low-visibility conditions on the moon's surface, posing a danger to ground-based travel.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        m.insert("Magda".to_string(), PlanetInfo {
            description: "Magda is the third-closest moon to Hurston. Although its temperature falls into ranges friendly to life, its nitrogen-methane atmosphere cannot be safely breathed. Enormous areas of blue-green dirt on Magda's surface are said to resemble oceans when the moon is viewed from orbit.".to_string(),
            difficulty: "⭐⭐".to_string(),
        });

        m.insert("Ita".to_string(), PlanetInfo {
            description: "Ita is the outermost moon of Hurston. Its dusty terrain is dotted with tall rock formations and craters from old meteor impacts.".to_string(),
            difficulty: "⭐⭐".to_string(),
        });

        m.insert("Crusader".to_string(), PlanetInfo {
            description: "Crusader is a low mass gas giant in the Stanton system. Rare among gas giants, Crusader's upper atmosphere is breathable by Humans and supports an active indigenous biosphere. The company settled Crusader via a complex network of floating platforms.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Cellin".to_string(), PlanetInfo {
            description: "Cellin is a volcanically active moon thronged with geysers that orbits Crusader, and is the planet's closest natural satellite. Its tenuous atmosphere contains trace amounts of oxygen, but not enough to support respiration.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Daymar".to_string(), PlanetInfo {
            description: "Daymar is Crusader's second closest natural satellite. Underground race Daymar Rally is held on Daymar every Standard Earth Year. In 2862, the Javelin-class destroyer UEES Flyssa crashed on Daymar after an illegal mining spacecraft it was pursuing unexpectedly exploded.".to_string(),
            difficulty: "⭐⭐".to_string(),
        });

        m.insert("Yela".to_string(), PlanetInfo {
            description: "Yela is the outermost natural satellite of Crusader, and is orbited by a ring of asteroids. A thin crust of water-ice on the moon's surface represents the cool demeanor of Yela, the oldest sibling in the Human children's story A Gift for Baba.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("ArcCorp".to_string(), PlanetInfo {
            description: "ArcCorp is the third planet from the sun of the Stanton system. In just 80 years all of the terrain of the planet has been sculpted, zoned, and built upon, leaving very little left for nature. It consists of layer after layer of factory; so many that today building additional structures is actually impossible.".to_string(),
            difficulty: "⭐".to_string(),
        });

        m.insert("Lyria".to_string(), PlanetInfo {
            description: "Lyria is a natural satellite of ArcCorp that orbits more closely to the planet than its sister moon, Wala. Water ice geysers and cryo-volcanoes force water vapor into Lyria's oxygen and ammonia atmosphere when they erupt.".to_string(),
            difficulty: "⭐⭐".to_string(),
        });

        m.insert("Wala".to_string(), PlanetInfo {
            description: "Wala is the second-closest moon of ArcCorp. Naturally-formed veins of blue-green crystal jutting from the moon's surface are visible from orbit. ArcCorp operates a number of surface mining facilities on Wala.".to_string(),
            difficulty: "⭐⭐".to_string(),
        });

        m.insert("microTech".to_string(), PlanetInfo {
            description: "microTech is the fourth and farthest planet from the sun in the Stanton system. Located at the outer edge of the system's habitable zone, microTech is mostly ice, with some cold-to-temperate areas of tundra. Its capital, New Babbage, is home to many corporations and startups.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        m.insert("Calliope".to_string(), PlanetInfo {
            description: "Calliope is a cold, windy moon of microTech, named after the mythical muse of epic poetry and eloquence from ancient Earth. Due to the high winds and extreme cold, weather suits worn by visitors to the planet are known to fail, particularly on the moon's dark side.".to_string(),
            difficulty: "⭐⭐⭐⭐⭐".to_string(),
        });

        m.insert("Clio".to_string(), PlanetInfo {
            description: "Clio is the second-closest moon to microTech, and is a popular destination for tourists due to its liquid oceans and pink and teal sky. It was named for the ancient Earth muse of history.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Euterpe".to_string(), PlanetInfo {
            description: "Euterpe is the third and smallest moon of microTech, named for the ancient Earth muse of music. Largely covered by ice sheets and frozen oceans of water, the landscape of the moon is dotted with pillars of rock formations.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        // Pyro System
        m.insert("Pyro I".to_string(), PlanetInfo {
            description: "Pyro I is the first planet from the sun of the Pyro system. Its thick, high-pressure atmosphere hides a strange landscape beset by powerful winds and lightning strikes.".to_string(),
            difficulty: "⭐⭐⭐⭐⭐".to_string(),
        });

        m.insert("Monox".to_string(), PlanetInfo {
            description: "Monox (Pyro II) is the second planet from the sun of the Pyro system. Nicknamed Monox for the deadly carbon monoxide in its atmosphere, it has a temperate climate but was deemed too dangerous for Human habitation.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        m.insert("Bloom".to_string(), PlanetInfo {
            description: "Bloom (Pyro III) is the third planet from the sun of the Pyro system. Cold but naturally habitable, it became the epicenter of mining endeavors in Pyro.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Pyro V".to_string(), PlanetInfo {
            description: "Pyro V is the fourth planet from the sun of the Pyro system. Residents of and travelers through the Pyro system are known to harvest hydrogen from Pyro V's green and yellow atmosphere.".to_string(),
            difficulty: "⭐⭐⭐⭐⭐".to_string(),
        });

        // Pyro V's moons
        m.insert("Ignis".to_string(), PlanetInfo {
            description: "Ignis is the innermost moon of the gas giant Pyro V. Its frigid average temperature and deadly atmosphere of sulfur dioxide creates an unfriendly environment for those who lack proper protective equipment.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        m.insert("Vatra".to_string(), PlanetInfo {
            description: "Vatra is the second moon from the gas giant Pyro V. Its thick, high-pressure atmosphere of nitrogen and methane supports an ecosystem of plants that subsist on hydrocarbons instead of carbon dioxide.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        m.insert("Adir".to_string(), PlanetInfo {
            description: "Adir is the third moon from the gas giant Pyro V. Jagged mountains and rocky hills dominate its crater-ridden landscape. The air is permeated with fine particles of dust.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Fairo".to_string(), PlanetInfo {
            description: "Fairo is the fourth moon from the gas giant Pyro V. A hotbed of seismic activity, Fairo experiences frequent earthquakes that disturb its deep oceans of brackish water.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Fuego".to_string(), PlanetInfo {
            description: "Fuego is the fifth moon from the gas giant Pyro V. Relatively temperate but still cold, it has a mostly carbon dioxide atmosphere with small percentages of nitrogen, sulfur dioxide, and oxygen.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Vuur".to_string(), PlanetInfo {
            description: "Vuur is the sixth moon from the gas giant Pyro V. Thanks to an increased greenhouse effect in its dense carbon dioxide atmosphere, it is unusually warm for a moon so distant from its sun.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m.insert("Pyro IV".to_string(), PlanetInfo {
            description: "Pyro IV is the outermost moon of Pyro V, and astronomers theorize that it was once the fourth planet from the sun of the Pyro system. It is on a long, slow collision course with Pyro V.".to_string(),
            difficulty: "⭐⭐⭐⭐".to_string(),
        });

        m.insert("Terminus".to_string(), PlanetInfo {
            description: "Terminus (Pyro VI) is the outermost world of the Pyro system. Its nitrogen-oxygen atmosphere is technically breathable, but contains high enough amounts of methane and carbon dioxide that it has a negative effect on one's health after extended exposure.".to_string(),
            difficulty: "⭐⭐⭐".to_string(),
        });

        m
    };
}
