use super::interests::INTERESTS;
use super::InterestPoint;
use super::Point;
use osmio::prelude::*;
use osmio::OSMObjBase;
use std::path::Path;

pub fn parse_osm_data<P: AsRef<Path>>(path: P) -> Vec<InterestPoint> {
    let reader = osmio::read_pbf(path).ok();
    reader
        .map(|mut reader| {
            let mut interests = Vec::new();
            for obj in reader.objects() {
                match obj {
                    osmio::obj_types::ArcOSMObj::Node(n) => {
                        n.lat_lon_f64().map(|(lat, lon)| {
                            for p in n.tags().filter_map(move |(k, v)| {
                                INTERESTS.get(&(k, v)).map(|i| InterestPoint {
                                    point: Point { x: lon, y: lat },
                                    interest: *i,
                                })
                            }) {
                                interests.push(p);
                            }
                        });
                    }
                    _ => {}
                }
            }
            interests
        })
        .unwrap_or_default()
}
