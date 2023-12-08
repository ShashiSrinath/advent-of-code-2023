use crate::util::fs_util::read_lines;

#[derive(PartialEq)]
enum MapSelector {
    None,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

struct RangeItem {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl RangeItem {
    fn new(destination_range_start: i64, source_range_start: i64, range_length: i64) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}

struct RangeData {
    data: Vec<RangeItem>,
}

impl RangeData {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, item: RangeItem) {
        self.data.push(item);
    }

    fn get_destination_index(&self, source_index: i64) -> i64 {
        let mut destination_index: Option<i64> = None;

        for i in &self.data {
            if source_index >= i.source_range_start
                && source_index < i.source_range_start + i.range_length
            {
                destination_index =
                    Some(i.destination_range_start + (source_index - i.source_range_start));
            }
        }

        match destination_index {
            None => source_index,
            Some(val) => val,
        }
    }
}

pub fn day_5_fertilizer() -> i64 {
    let lines = read_lines("./inputs/day_5/values.txt").unwrap();

    let mut seeds: Vec<i64> = Vec::new();

    let mut seed_to_soil_map: RangeData = RangeData::new();
    let mut soil_to_fertilizer_map: RangeData = RangeData::new();
    let mut fertilizer_to_water_map: RangeData = RangeData::new();
    let mut water_to_light_map: RangeData = RangeData::new();
    let mut light_to_temperature_map: RangeData = RangeData::new();
    let mut temperature_to_humidity_map: RangeData = RangeData::new();
    let mut humidity_to_location_map: RangeData = RangeData::new();

    let mut current_switch = MapSelector::None;

    for (index, line) in lines.enumerate() {
        let line = line.unwrap();

        if index == 0 {
            seeds = line
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| v.parse().unwrap())
                .collect();

            continue;
        }

        if line.is_empty() {
            current_switch = MapSelector::None;
            continue;
        }

        if current_switch == MapSelector::None {
            if line == "seed-to-soil map:" {
                current_switch = MapSelector::SeedToSoil;
                continue;
            }
            if line == "soil-to-fertilizer map:" {
                current_switch = MapSelector::SoilToFertilizer;
                continue;
            }
            if line == "fertilizer-to-water map:" {
                current_switch = MapSelector::FertilizerToWater;
                continue;
            }
            if line == "water-to-light map:" {
                current_switch = MapSelector::WaterToLight;
                continue;
            }
            if line == "light-to-temperature map:" {
                current_switch = MapSelector::LightToTemperature;
                continue;
            }
            if line == "temperature-to-humidity map:" {
                current_switch = MapSelector::TemperatureToHumidity;
                continue;
            }
            if line == "humidity-to-location map:" {
                current_switch = MapSelector::HumidityToLocation;
                continue;
            }
        }

        // parse line
        let map_inputs: Vec<i64> = line
            .trim()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect();

        let destination_range_start = map_inputs.get(0).unwrap();
        let source_range_start = map_inputs.get(1).unwrap();
        let range_length = map_inputs.get(2).unwrap();

        match current_switch {
            MapSelector::SeedToSoil => {
                seed_to_soil_map.push(RangeItem::new(
                    *destination_range_start,
                    *source_range_start,
                    *range_length,
                ));
            }
            MapSelector::SoilToFertilizer => {
                soil_to_fertilizer_map.push(RangeItem::new(
                    *destination_range_start,
                    *source_range_start,
                    *range_length,
                ));
            }
            MapSelector::FertilizerToWater => {
                fertilizer_to_water_map.push(RangeItem::new(
                    *destination_range_start,
                    *source_range_start,
                    *range_length,
                ));
            }
            MapSelector::WaterToLight => {
                water_to_light_map.push(RangeItem::new(
                    *destination_range_start,
                    *source_range_start,
                    *range_length,
                ));
            }
            MapSelector::LightToTemperature => {
                light_to_temperature_map.push(RangeItem::new(
                    *destination_range_start,
                    *source_range_start,
                    *range_length,
                ));
            }
            MapSelector::TemperatureToHumidity => {
                temperature_to_humidity_map.push(RangeItem::new(
                    *destination_range_start,
                    *source_range_start,
                    *range_length,
                ));
            }
            MapSelector::HumidityToLocation => {
                humidity_to_location_map.push(RangeItem::new(
                    *destination_range_start,
                    *source_range_start,
                    *range_length,
                ));
            }
            _ => {}
        }
    }

    let mut lowest_location_number: Option<i64> = None;
    for seed in seeds {
        let soil = seed_to_soil_map.get_destination_index(seed);
        let fertilizer = soil_to_fertilizer_map.get_destination_index(soil);
        let water = fertilizer_to_water_map.get_destination_index(fertilizer);
        let light = water_to_light_map.get_destination_index(water);
        let temperature = light_to_temperature_map.get_destination_index(light);
        let humidity = temperature_to_humidity_map.get_destination_index(temperature);
        let location = humidity_to_location_map.get_destination_index(humidity);

        match lowest_location_number {
            None => lowest_location_number = Some(location),
            Some(lowest_location) => {
                if location < lowest_location {
                    lowest_location_number = Some(location);
                }
            }
        }
    }

    lowest_location_number.unwrap()
}
