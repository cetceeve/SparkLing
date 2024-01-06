use std::collections::{HashSet, HashMap};
use std::io::Write;

use crate::Vehicle;
use chrono::{NaiveDateTime, Timelike, Datelike, NaiveTime, Duration};

use super::ProcessingStep;

/// A token in our Vocabulary.
/// 0-30: time_delta -15m to +15m
/// 31: sos
/// 32: eos
/// 33: pad
/// 34-47: route_token: route_id + direction_id
/// 48-149: stop_name ['Kungsträdgården', 'T-Centralen', 'Rådhuset', 'Fridhemsplan', 'Stadshagen', 'Västra skogen', 'Huvudsta', 'Solna strand', 'Sundbybergs centrum', 'Duvbo', 'Rissne', 'Rinkeby', 'Tensta', 'Hjulsta', 'Solna centrum', 'Näckrosen', 'Hallonbergen', 'Kymlinge norrut', 'Kista', 'Husby', 'Akalla', 'Kymlinge söderut', 'Norsborg', 'Hallunda', 'Alby', 'Fittja', 'Masmo', 'Vårby gård', 'Vårberg', 'Skärholmen', 'Sätra', 'Bredäng', 'Mälarhöjden', 'Axelsberg', 'Örnsberg', 'Aspudden', 'Liljeholmen', 'Hornstull', 'Zinkensdamm', 'Mariatorget', 'Slussen', 'Gamla stan', 'Östermalmstorg', 'Karlaplan', 'Gärdet', 'Ropsten', 'Mörby centrum', 'Danderyds sjukhus', 'Bergshamra', 'Universitetet', 'Tekniska högskolan', 'Stadion', 'Fruängen', 'Västertorp', 'Hägerstensåsen', 'Telefonplan', 'Midsommarkransen', 'Skarpnäck', 'Bagarmossen', 'Kärrtorp', 'Björkhagen', 'Hammarbyhöjden', 'Skärmarbrink', 'Gullmarsplan', 'Skanstull', 'Medborgarplatsen', 'Hötorget', 'Rådmansgatan', 'Odenplan', 'S:t Eriksplan', 'Thorildsplan', 'Kristineberg', 'Alvik', 'Stora mossen', 'Abrahamsberg', 'Brommaplan', 'Åkeshov', 'Ängbyplan', 'Islandstorget', 'Blackeberg', 'Råcksta', 'Vällingby', 'Johannelund', 'Hässelby gård', 'Hässelby strand', 'Farsta strand', 'Farsta', 'Hökarängen', 'Gubbängen', 'Tallkrogen', 'Skogskyrkogården', 'Sandsborg', 'Blåsut', 'Hagsätra', 'Rågsved', 'Högdalen', 'Bandhagen', 'Stureby', 'Svedmyra', 'Sockenplan', 'Enskede gård', 'Globen']
/// 150-156: weekday
/// 157-180: hour_of_day
/// 181: skip token, indicates for inference which values to predict
pub struct Token(u64);

/// A complete sequence, used for training of our model
/// pattern:  sos route_token weekday hour_of_day stop_name time_delta ... stop_name time_delta eos pad pad pad ...
///
/// The max trip length in the schedule is 35 stops,
/// so the max sequence length will be 4 + 35*2 + 1 = 75
pub struct TrainingSequence(Vec<Token>);

/// Takes input in seconds
pub fn tokenize_time_delta(delta: i64) -> Token {
    Token(((delta / 60).max(-15).min(15) + 15) as u64)
}

pub fn tokenize_route(id: &str, direction_id: u8) -> Token {
    let routes = vec!["9011001001000000".to_string(), "9011001001100000".to_string(), "9011001001300000".to_string(), "9011001001400000".to_string(), "9011001001700000".to_string(), "9011001001800000".to_string(), "9011001001900000".to_string()];
    Token(34 + 7 * direction_id as u64 + routes.iter().position(|s| *s == *id).unwrap() as u64)
}

pub fn tokenize_stop_name(name: &str) -> Token {
    let stops = vec!["Kungsträdgården".to_string(), "T-Centralen".to_string(), "Rådhuset".to_string(), "Fridhemsplan".to_string(), "Stadshagen".to_string(), "Västra skogen".to_string(), "Huvudsta".to_string(), "Solna strand".to_string(), "Sundbybergs centrum".to_string(), "Duvbo".to_string(), "Rissne".to_string(), "Rinkeby".to_string(), "Tensta".to_string(), "Hjulsta".to_string(), "Solna centrum".to_string(), "Näckrosen".to_string(), "Hallonbergen".to_string(), "Kymlinge norrut".to_string(), "Kista".to_string(), "Husby".to_string(), "Akalla".to_string(), "Kymlinge söderut".to_string(), "Norsborg".to_string(), "Hallunda".to_string(), "Alby".to_string(), "Fittja".to_string(), "Masmo".to_string(), "Vårby gård".to_string(), "Vårberg".to_string(), "Skärholmen".to_string(), "Sätra".to_string(), "Bredäng".to_string(), "Mälarhöjden".to_string(), "Axelsberg".to_string(), "Örnsberg".to_string(), "Aspudden".to_string(), "Liljeholmen".to_string(), "Hornstull".to_string(), "Zinkensdamm".to_string(), "Mariatorget".to_string(), "Slussen".to_string(), "Gamla stan".to_string(), "Östermalmstorg".to_string(), "Karlaplan".to_string(), "Gärdet".to_string(), "Ropsten".to_string(), "Mörby centrum".to_string(), "Danderyds sjukhus".to_string(), "Bergshamra".to_string(), "Universitetet".to_string(), "Tekniska högskolan".to_string(), "Stadion".to_string(), "Fruängen".to_string(), "Västertorp".to_string(), "Hägerstensåsen".to_string(), "Telefonplan".to_string(), "Midsommarkransen".to_string(), "Skarpnäck".to_string(), "Bagarmossen".to_string(), "Kärrtorp".to_string(), "Björkhagen".to_string(), "Hammarbyhöjden".to_string(), "Skärmarbrink".to_string(), "Gullmarsplan".to_string(), "Skanstull".to_string(), "Medborgarplatsen".to_string(), "Hötorget".to_string(), "Rådmansgatan".to_string(), "Odenplan".to_string(), "S:t Eriksplan".to_string(), "Thorildsplan".to_string(), "Kristineberg".to_string(), "Alvik".to_string(), "Stora mossen".to_string(), "Abrahamsberg".to_string(), "Brommaplan".to_string(), "Åkeshov".to_string(), "Ängbyplan".to_string(), "Islandstorget".to_string(), "Blackeberg".to_string(), "Råcksta".to_string(), "Vällingby".to_string(), "Johannelund".to_string(), "Hässelby gård".to_string(), "Hässelby strand".to_string(), "Farsta strand".to_string(), "Farsta".to_string(), "Hökarängen".to_string(), "Gubbängen".to_string(), "Tallkrogen".to_string(), "Skogskyrkogården".to_string(), "Sandsborg".to_string(), "Blåsut".to_string(), "Hagsätra".to_string(), "Rågsved".to_string(), "Högdalen".to_string(), "Bandhagen".to_string(), "Stureby".to_string(), "Svedmyra".to_string(), "Sockenplan".to_string(), "Enskede gård".to_string(), "Globen".to_string()];
    Token(48 + stops.iter().position(|s| *s == *name).unwrap() as u64)
}

pub fn tokenize_time(ts: NaiveDateTime) -> (Token, Token) {
    (
        Token(150 + ts.weekday().num_days_from_monday() as u64),
        Token(157 + ts.hour() as u64)
    )
}

pub struct InferenceFeatureExtractor {
    highest_reached_stop_idx: HashMap<String, usize>,
}

impl InferenceFeatureExtractor {
    pub fn init() -> Self {
        Self {
            highest_reached_stop_idx: Default::default(),
        }
    }
}

impl ProcessingStep for InferenceFeatureExtractor {
    fn apply(&mut self, vehicle: &mut Vehicle, _low_watermark: u64) -> (bool, Option<(String, Vec<u8>)>) {
        // get metadata if there
        if let Some(ref vehicle_metadata) = vehicle.metadata {
            // we only deal with metros
            if vehicle_metadata.route_type != Some(401) {
                return (true, None)
            }
            if let (Some(real_stop_times), Some(stops)) = (&vehicle_metadata.real_stop_times, &vehicle_metadata.stops) {
                if stops.len() < 3 { // sequence too short, we don't predict here
                    return (true, None)
                }
                let mut new_highest_idx = 0;
                for i in (0..stops.len()).rev() {
                    if real_stop_times[i] != None {
                        new_highest_idx = i;
                        break
                    }
                }
                let old_highest_idx = self.highest_reached_stop_idx.get(&vehicle.id);
                if Some(&new_highest_idx) == old_highest_idx {
                    return (true, None) // we already predicted here, need to reach next stop first
                } else { // otherwise, we update the highest index we predicted at and do a prediction
                    self.highest_reached_stop_idx.insert(vehicle.id.clone(), new_highest_idx);
                }
                if stops.iter().any(|x| x.arrival_time.starts_with("24") || x.arrival_time.starts_with("25") || x.arrival_time.starts_with("26") || x.arrival_time.starts_with("27") || x.arrival_time.starts_with("28") || x.arrival_time.starts_with("29") || x.arrival_time.starts_with("30")) {
                    return (true, None) // we don't deal with funny timestamps
                }

                // convert scheduled stop times to useful timestamps
                let end_date = NaiveDateTime::from_timestamp_millis(vehicle.timestamp as i64 * 1000).unwrap().date();
                let mut scheduled_times = stops.iter().map(|x| {
                    end_date.and_time(NaiveTime::parse_from_str(&x.arrival_time, "%H:%M:%S").expect(format!("Failed to parse: {:x?}", x).as_str()))
                }).collect::<Vec<NaiveDateTime>>();
                // if the trip ended past midnight, we need to fix the scheduled dates for stops before midnight
                for i in (1..scheduled_times.len()).rev() {
                    if scheduled_times[i] < scheduled_times[i-1] {
                        scheduled_times[i-1] -= Duration::days(1);
                    }
                }
                // we need to subtract 1h for some reason, maybe trafiklab uses timezones inconsistently
                let scheduled_timestamps = scheduled_times.iter().map(|x| x.timestamp() - 3600).collect::<Vec<i64>>();

                // construct sequence
                let route_token = tokenize_route(vehicle_metadata.route_id.as_ref().unwrap(), vehicle_metadata.direction_id.unwrap());
                let (day_token, hour_token) = tokenize_time(scheduled_times[0]);
                let mut sequence = TrainingSequence(vec![Token(31), route_token, day_token, hour_token]);
                sequence.0.push(tokenize_stop_name(&stops[0].stop_name));
                sequence.0.push(tokenize_time_delta(0)); // 0m time_delta

                // For inference, we generate the same sequence as for training (without <eos>), but we replace the
                // tokens we want to predict with the <pad> token 33. The inference then needs to
                // deconstruct this correctly before feeding it to the model.
                let mut prev_delay = 0;
                for (i, stop) in stops[1..].iter().enumerate() {
                    sequence.0.push(tokenize_stop_name(&stop.stop_name));
                    if i <= new_highest_idx {
                        if let Some(real_time) = real_stop_times[i] {
                            let new_delay = real_time as i64 - scheduled_timestamps[i];
                            sequence.0.push(tokenize_time_delta(new_delay - prev_delay));
                            prev_delay = new_delay;
                        } else {
                            sequence.0.push(tokenize_time_delta(0));
                        }
                    } else {
                        sequence.0.push(Token(181)); // <skip> in places we want to predict
                    }
                }
                while sequence.0.len() < 75 {
                    sequence.0.push(Token(33)); // pad to 75
                }
                // serialize and return to be sent to redis
                let mut serialized: Vec<u8> = vec![];
                write!(&mut serialized, "{}:{}", &vehicle.id, sequence.0[0].0).unwrap();
                for token in sequence.0[1..].iter() {
                    write!(&mut serialized, ",{}", token.0).unwrap();
                }
                return (true, Some(("lstm-inference-features".to_string(), serialized)))
            }
        }
        (true, None)
    }
}

pub struct TrainingFeatureExtractor {
    duplicate_filter: HashSet<String>,
}

impl TrainingFeatureExtractor {
    pub fn init() -> Self {
        Self {
            duplicate_filter: Default::default(),
        }
    }
}

impl ProcessingStep for TrainingFeatureExtractor {
    fn apply(&mut self, vehicle: &mut Vehicle, _low_watermark: u64) -> (bool, Option<(String, Vec<u8>)>) {
        // get metadata if there
        if let Some(ref vehicle_metadata) = vehicle.metadata {
            // we only deal with metros
            if vehicle_metadata.route_type != Some(401) {
                return (true, None)
            }
            if let (Some(real_stop_times), Some(stops)) = (&vehicle_metadata.real_stop_times, &vehicle_metadata.stops) {
                if real_stop_times.len() < 3 || *real_stop_times.last().unwrap() == None || real_stop_times.iter().filter(|x| **x == None).count() > stops.len() / 2 {
                    return (true, None) // we only generate training features once we reached the last stop
                }
                if self.duplicate_filter.contains(vehicle.trip_id.as_ref().unwrap()) {
                    return (true, None) // only emit features once per trip
                }
                self.duplicate_filter.insert(vehicle.trip_id.clone().unwrap());
                if stops.iter().any(|x| x.arrival_time.starts_with("24") || x.arrival_time.starts_with("25") || x.arrival_time.starts_with("26") || x.arrival_time.starts_with("27") || x.arrival_time.starts_with("28") || x.arrival_time.starts_with("29") || x.arrival_time.starts_with("30")) {
                    return (true, None) // we don't deal with funny timestamps
                }

                // convert scheduled stop times to useful timestamps
                let end_date = NaiveDateTime::from_timestamp_millis(vehicle.timestamp as i64 * 1000).unwrap().date();
                let mut scheduled_times = stops.iter().map(|x| {
                    end_date.and_time(NaiveTime::parse_from_str(&x.arrival_time, "%H:%M:%S").expect(format!("Failed to parse: {:x?}", x).as_str()))
                }).collect::<Vec<NaiveDateTime>>();
                // if the trip ended past midnight, we need to fix the scheduled dates for stops before midnight
                for i in (1..scheduled_times.len()).rev() {
                    if scheduled_times[i] < scheduled_times[i-1] {
                        scheduled_times[i-1] -= Duration::days(1);
                    }
                }
                // we need to subtract 1h for some reason, maybe trafiklab uses timezones inconsistently
                let scheduled_timestamps = scheduled_times.iter().map(|x| x.timestamp() - 3600).collect::<Vec<i64>>();

                // construct sequence
                let route_token = tokenize_route(vehicle_metadata.route_id.as_ref().unwrap(), vehicle_metadata.direction_id.unwrap());
                let (day_token, hour_token) = tokenize_time(scheduled_times[0]);
                let mut sequence = TrainingSequence(vec![Token(31), route_token, day_token, hour_token]);
                sequence.0.push(tokenize_stop_name(&stops[0].stop_name));
                sequence.0.push(tokenize_time_delta(0)); // 0m time_delta

                let mut prev_delay = 0;
                for (i, stop) in stops[1..].iter().enumerate() {
                    sequence.0.push(tokenize_stop_name(&stop.stop_name));
                    if let Some(real_time) = real_stop_times[i] {
                        let new_delay = real_time as i64 - scheduled_timestamps[i];
                        sequence.0.push(tokenize_time_delta(new_delay - prev_delay));
                        prev_delay = new_delay;
                    } else {
                        sequence.0.push(tokenize_time_delta(0));
                    }
                }
                sequence.0.push(Token(32)); // end of sequence
                while sequence.0.len() < 75 {
                    sequence.0.push(Token(33)); // pad to 75
                }
                // serialize and return to be sent to redis
                let mut serialized: Vec<u8> = vec![];
                write!(&mut serialized, "{}", sequence.0[0].0).unwrap();
                for token in sequence.0[1..].iter() {
                    write!(&mut serialized, ",{}", token.0).unwrap();
                }
                return (true, Some(("lstm-training-features".to_string(), serialized)))
            }
        }
        (true, None)
    }
}
