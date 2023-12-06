use std::ops::Range;

pub type Seeds = Vec<u64>;

#[derive(Debug)]
pub struct MapEntry {
    pub source: Range<u64>,
    pub dest: Range<u64>,
}

#[derive(Debug)]
pub struct Map {
    pub name: String,
    pub entries: Vec<MapEntry>
}

impl Map {
    fn lookup(&self, source: u64) -> u64 {
        for entry in &self.entries {
            if entry.source.contains(&source) {
                return source - entry.source.start + entry.dest.start;
            }
        }
        source
    }
}

#[derive(Debug)]
pub struct Maps {
    pub seed_to_soil: Map,
    pub soil_to_fertilizer: Map,
    pub fertilizer_to_water: Map,
    pub water_to_light: Map,
    pub light_to_temperature: Map,
    pub temperature_to_humidity: Map,
    pub humidity_to_location: Map   
}

impl Maps {
    #[must_use]
    pub fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.lookup(seed);
        let fertilizer = self.soil_to_fertilizer.lookup(soil);
        let water = self.fertilizer_to_water.lookup(fertilizer);
        let light = self.water_to_light.lookup(water);
        let temperature = self.light_to_temperature.lookup(light);
        let humidity = self.temperature_to_humidity.lookup(temperature);
        self.humidity_to_location.lookup(humidity)
    }
}