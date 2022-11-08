use std::{collections::HashSet, io::Write};

use crate::{bounding_box, interests::InterestPoint, Point};

fn save_path<W: Write>(writer: &mut W, p: &[Point], stroke: &str) -> std::io::Result<()> {
    write!(
        writer,
        "<polyline fill='none' stroke='{}' stroke-width='0.2%' points='",
        stroke
    )?;
    p.iter()
        .try_for_each(|p| write!(writer, "{},{} ", p.x, p.y))?;
    writeln!(writer, "'/>")?;
    Ok(())
}

// save svg file from given path and interest points.
// useful for debugging path simplification and previewing traces.
pub fn save_svg<'a, W: Write, I: IntoIterator<Item = &'a InterestPoint>>(
    mut writer: W,
    p: &[Point],
    rp: &[Point],
    interest_points: I,
    waypoints: &HashSet<Point>,
) -> std::io::Result<()> {
    let (xmin, ymin, xmax, ymax) = bounding_box(p);

    writeln!(
        &mut writer,
        "<svg width='800' height='600' viewBox='{} {} {} {}'>",
        xmin,
        ymin,
        xmax - xmin,
        ymax - ymin
    )?;
    write!(
        &mut writer,
        "<rect fill='white' x='{}' y='{}' width='{}' height='{}'/>",
        xmin,
        ymin,
        xmax - xmin,
        ymax - ymin
    )?;

    writeln!(
        &mut writer,
        "<g transform='translate(0, {}) scale(1,-1)'>",
        ymin + ymax
    )?;
    save_path(&mut writer, &p, "red")?;
    save_path(&mut writer, &rp, "black")?;

    for point in interest_points {
        writeln!(
            &mut writer,
            "<circle cx='{}' cy='{}' fill='{}' r='0.8%'/>",
            point.point.x,
            point.point.y,
            point.color(),
        )?;
    }

    waypoints.iter().try_for_each(|p| {
        writeln!(
            &mut writer,
            "<circle cx='{}' cy='{}' fill='black' r='0.3%'/>",
            p.x, p.y,
        )
    })?;

    writeln!(&mut writer, "</g>")?;
    writeln!(&mut writer, "</svg>")?;
    Ok(())
}
