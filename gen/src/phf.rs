use crate::Value;

#[derive(Debug, Clone)]
pub struct Phf {
    pub params: Vec<u32>,
    pub values: Vec<u32>,
}

fn jenkins_mix(mut value: u32) -> u32 {
    value = value.wrapping_add(0x7ed55d16).wrapping_add(value << 12);
    value = (value ^ 0xc761c23c) ^ (value >> 19);
    value = value.wrapping_add(0x165667b1).wrapping_add(value << 5);
    value = value.wrapping_add(0xd3a2646c) ^ (value << 9);
    value = value.wrapping_add(0xfd7046c5).wrapping_add(value << 3);
    value = (value ^ 0xb55a4f09) ^ (value >> 16);
    value
}

fn jenkins_hash(init: u32, value: &[u8]) -> u32 {
    let mut h = init;
    let mut idx = 0usize;
    while idx < value.len() {
        h = h.wrapping_add(value[idx] as u32);
        h = h.wrapping_add(h << 10);
        h ^= h >> 6;
        idx += 1;
    }
    h = h.wrapping_add(h << 3);
    h ^= h >> 11;
    h = h.wrapping_add(h << 15);
    h
}

fn hash_combine<const COUNT: usize>(hashes: [u32; COUNT]) -> u32 {
    let mut h = hashes[0];
    let mut idx = 1usize;
    while idx < COUNT {
        h ^= hashes[idx]
            .wrapping_add(0x9e3779b9)
            .wrapping_add(h << 6)
            .wrapping_add(h >> 2);
        idx += 1;
    }
    h
}

fn value_hash(param: u32, key: &Value) -> u32 {
    let h = jenkins_mix(param);
    match key {
        Value::Null => hash_combine([0, h]),
        Value::Bool(false) => hash_combine([1, h]),
        Value::Bool(true) => hash_combine([2, h]),
        Value::UInt(val) => hash_combine([
            3,
            h,
            (*val >> 96) as u32,
            ((*val >> 64) & 0xffffffff) as u32,
            ((*val >> 32) & 0xffffffff) as u32,
            ((*val & 0xffffffff) as u32),
        ]),
        Value::Int(val) => {
            let seed = if *val < 0 { 4 } else { 3 };
            let val = u128::from_ne_bytes((*val).to_ne_bytes());
            hash_combine([
                seed,
                h,
                (val >> 96) as u32,
                ((val >> 64) & 0xffffffff) as u32,
                ((val >> 32) & 0xffffffff) as u32,
                ((val & 0xffffffff) as u32),
            ])
        }
        Value::Float(val) => {
            if *val < 0.0f64 {
                if ((*val as i128) as f64) == *val {
                    let val = u128::from_ne_bytes((*val as i128).to_ne_bytes());
                    hash_combine([
                        4,
                        h,
                        (val >> 96) as u32,
                        ((val >> 64) & 0xffffffff) as u32,
                        ((val >> 32) & 0xffffffff) as u32,
                        ((val & 0xffffffff) as u32),
                    ])
                } else {
                    let val = u64::from_ne_bytes((*val).to_ne_bytes());
                    hash_combine([5, h, (val >> 32) as u32, (val & 0xffffffff) as u32])
                }
            } else {
                if ((*val as u128) as f64) == *val {
                    let val = *val as u128;
                    hash_combine([
                        3,
                        h,
                        (val >> 96) as u32,
                        ((val >> 64) & 0xffffffff) as u32,
                        ((val >> 32) & 0xffffffff) as u32,
                        ((val & 0xffffffff) as u32),
                    ])
                } else {
                    let val = u64::from_ne_bytes((*val).to_ne_bytes());
                    hash_combine([5, h, (val >> 32) as u32, (val & 0xffffffff) as u32])
                }
            }
        }
        Value::Date(date) => {
            hash_combine([6, date.year as u32, date.month as u32, date.day as u32])
        }
        Value::Time(time) => hash_combine([
            7,
            h,
            time.hour as u32,
            time.minute as u32,
            time.second as u32,
            time.nanosecond,
        ]),
        Value::DateTime(dt) => {
            let h = hash_combine([
                8,
                h,
                dt.date.year as u32,
                dt.date.month as u32,
                dt.date.day as u32,
                dt.time.hour as u32,
                dt.time.minute as u32,
                dt.time.second as u32,
                dt.time.nanosecond,
            ]);
            if let Some(offset) = &dt.offset {
                hash_combine([h, 1, u16::from_ne_bytes((*offset).to_ne_bytes()) as u32])
            } else {
                hash_combine([h, 0])
            }
        }
        Value::Str(s) => hash_combine([9, jenkins_hash(h, s.as_bytes())]),
        Value::Bytes(b) => hash_combine([10, jenkins_hash(h, b)]),
        Value::Array(array) => {
            let mut h = hash_combine([11, h]);
            let mut idx = 0usize;
            while idx < array.len() {
                h = value_hash(h, &array[idx]);
                idx += 1;
            }
            h
        }
        Value::Object(obj) => {
            let mut h = hash_combine([12, h]);
            let mut idx = 0usize;
            while idx < obj.len() {
                let entry = &obj[idx];
                h = jenkins_hash(h, entry.0.as_bytes());
                h = value_hash(h, &entry.1);
                idx += 1;
            }
            h
        }
        Value::Map(map) => {
            let mut h = hash_combine([13, h]);
            let mut idx = 0usize;
            while idx < map.len() {
                let entry = &map[idx];
                h = value_hash(h, &entry.0);
                h = value_hash(h, &entry.1);
                idx += 1;
            }
            h
        }
    }
}

impl Phf {
    pub fn build_object(obj: &[(String, Value)]) -> Self {
        if obj.is_empty() {
            return Self {
                params: Vec::new(),
                values: Vec::new(),
            };
        }

        let mut params = vec![0u32; obj.len()];
        let mut values = vec![obj.len() as u32; obj.len()];

        let mut buckets = Vec::with_capacity(obj.len());
        buckets.resize(obj.len(), Vec::new());

        let mut idx = 0u32;
        for (key, _) in obj.iter() {
            buckets[(jenkins_hash(jenkins_mix(0), key.as_bytes()) as usize) % obj.len()].push(idx);
            idx += 1;
        }

        let mut sorted_buckets: Vec<usize> = (0..obj.len()).into_iter().collect();
        sorted_buckets.sort_by(|l, r| std::cmp::Ord::cmp(&buckets[*r].len(), &buckets[*l].len()));

        let mut slots: Vec<u32> = Vec::with_capacity(obj.len());
        for idx in 0..obj.len() {
            let bucket = &buckets[sorted_buckets[idx]];
            if bucket.is_empty() {
                continue;
            }

            let mut d: u32 = 1;
            let mut item: usize = 0;
            slots.clear();

            while (item as usize) < bucket.len() {
                let slot = (jenkins_hash(jenkins_mix(d), obj[bucket[item] as usize].0.as_bytes())
                    as usize
                    % obj.len()) as u32;
                if values[slot as usize] != obj.len() as u32 {
                    d += 1;
                    item = 0;
                    for s in slots.iter() {
                        values[*s as usize] = obj.len() as u32;
                    }
                    slots.clear();
                } else {
                    slots.push(slot);
                    values[slot as usize] = bucket[item];
                    item += 1;
                }
            }

            params[sorted_buckets[idx]] = d;
        }

        Self { params, values }
    }

    pub fn build_map(map: &[(Value, Value)]) -> Self {
        if map.is_empty() {
            return Self {
                params: Vec::new(),
                values: Vec::new(),
            };
        }

        let mut params = vec![0u32; map.len()];
        let mut values = vec![map.len() as u32; map.len()];

        let mut buckets = Vec::with_capacity(map.len());
        buckets.resize(map.len(), Vec::new());

        let mut idx = 0u32;
        for (key, _) in map.iter() {
            buckets[(value_hash(jenkins_mix(0), key) as usize) % map.len()].push(idx);
            idx += 1;
        }

        let mut sorted_buckets: Vec<usize> = (0..map.len()).into_iter().collect();
        sorted_buckets.sort_by(|l, r| std::cmp::Ord::cmp(&buckets[*r].len(), &buckets[*l].len()));

        let mut slots: Vec<u32> = Vec::with_capacity(map.len());
        for idx in 0..map.len() {
            let bucket = &buckets[sorted_buckets[idx]];
            if bucket.is_empty() {
                continue;
            }

            let mut d: u32 = 1;
            let mut item: usize = 0;
            slots.clear();

            while (item as usize) < bucket.len() {
                let slot = ((value_hash(jenkins_mix(d), &map[bucket[item] as usize].0) as usize)
                    % map.len()) as u32;
                if values[slot as usize] != map.len() as u32 {
                    d += 1;
                    item = 0;
                    for s in slots.iter() {
                        values[*s as usize] = map.len() as u32;
                    }
                    slots.clear();
                } else {
                    slots.push(slot);
                    values[slot as usize] = bucket[item];
                    item += 1;
                }
            }

            params[sorted_buckets[idx]] = d;
        }

        Self { params, values }
    }
}
