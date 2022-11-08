use gpconv::convert_gpx_files;

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or("m.gpx".to_string());
    let interests;

    #[cfg(feature = "osm")]
    {
        interests = std::env::args()
            .nth(2)
            .map(|osm_file| gpconv::parse_osm_data(osm));
    }
    #[cfg(not(feature = "osm"))]
    {
        interests = None;
    }

    convert_gpx_files(&input_file, interests);
}
