use std::io::BufWriter;

use gpconv::convert_gpx_files;

#[tokio::main]
async fn main() {
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

    let svg_writer = BufWriter::new(std::fs::File::create("output.svg").unwrap());
    convert_gpx_files(&input_file, svg_writer, interests).await;
}
