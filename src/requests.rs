use crate::{
    bounding_box,
    interests::{InterestPoint, INTERESTS},
    Point,
};
use xml::reader::{EventReader, XmlEvent};

pub async fn download_openstreetmap_interests(
    points: &[Point],
) -> Result<Vec<InterestPoint>, Box<dyn std::error::Error>> {
    let xml = download_openstreetmap_interests_xml(points).await?;
    Ok(parse_osm_xml(&xml))
}

pub async fn download_openstreetmap_interests_xml(
    points: &[Point],
) -> Result<String, Box<dyn std::error::Error>> {
    println!("downloading interests from openstreetmap");
    let (xmin, ymin, xmax, ymax) = bounding_box(points);
    let interests_nodes = INTERESTS
        .keys()
        .map(|(k, v)| format!("node[\"{}\"=\"{}\"];", k, v))
        .collect::<String>();
    let query = format!(
        "https://overpass-api.de/api/interpreter?data=
        [bbox: {}, {}, {}, {}];
        (
        {}
        );
        out body;",
        ymin, xmin, ymax, xmax, interests_nodes
    );
    let client = reqwest::Client::builder()
        //.user_agent("osm-geo-mapper")
        .build()?;
    let response = client.get(&query).send().await?;
    let result = response.text().await?;
    Ok(result)
}

pub fn parse_osm_xml(xml: &str) -> Vec<InterestPoint> {
    let parser = EventReader::new(xml.as_bytes());
    let mut current_node = None;
    let mut interest_points = Vec::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "node" {
                    let mut lon = None;
                    let mut lat = None;
                    for attribute in &attributes {
                        if attribute.name.local_name == "lon" {
                            lon = attribute.value.parse::<f64>().ok();
                            if lat.is_some() {
                                break;
                            }
                        } else if attribute.name.local_name == "lat" {
                            lat = attribute.value.parse::<f64>().ok();
                            if lon.is_some() {
                                break;
                            }
                        }
                    }
                    if let Some(lon) = lon {
                        if let Some(lat) = lat {
                            current_node = Some(Point { x: lon, y: lat })
                        }
                    }
                } else if name.local_name == "tag" {
                    let mut key = None;
                    let mut value = None;
                    for attribute in &attributes {
                        if attribute.name.local_name == "k" {
                            key = Some(&attribute.value);
                        } else if attribute.name.local_name == "v" {
                            value = Some(&attribute.value);
                        }
                    }
                    if let Some(key) = key {
                        if let Some(value) = value {
                            INTERESTS
                                .get(&(key, value))
                                .and_then(|&interest| {
                                    current_node.map(|point| InterestPoint { point, interest })
                                })
                                .map(|interest_point| interest_points.push(interest_point));
                        }
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "node" {
                    current_node = None;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    interest_points
}
