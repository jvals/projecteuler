use plotters::prelude::*;
use std::iter::repeat_with;
use plotters::coord::Shift;

fn generate_triangles() -> Vec<Vec<(i32, i32)>> {
    const X_MAX: i32 = 50;
    const Y_MAX: i32 = 50;
    const E: f64 = 0.01;

    let mut total = 0;
    let mut triangles = Vec::new();
    for y1 in 0..=Y_MAX {
        for x1 in 0..=X_MAX {
            for y2 in 0..=Y_MAX {
                for x2 in 0..=X_MAX {
                    if (y1 == y2 && x1 == x2) || (y1 == 0 && x1 == 0) || (y2 == 0 && x2 == 0) {
                        continue;
                    }
                    let x3 = 0;
                    let y3 = 0;

                    if is_pythagorean(x1, y1, x2, y2, x3, y3, E) {
                        triangles.push(vec![(0, 0), (x1, y1), (x2, y2)]);
                        println!("{} A=({},{}), B=({},{}), C=({},{})", total, 0, 0, x1, y1, x2, y2);
                        total += 1;
                    }
                }
            }
        }
    }
    println!("Found {} triangles. The answer is half of that: {}", total, total / 2);
    triangles
}

fn is_pythagorean(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, e: f64) -> bool {
    let l1 = length(x1, y1, x3, y3);
    let l2 = length(x2, y2, x3, y3);
    let l3 = length(x1, y1, x2, y2);
    let mut l = vec![l1, l2, l3];
    l.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let area = area(y1, x1, y2, x2, x3, y3);
    if area < e {
        return false;
    }
    ((l[0].powi(2) + l[1].powi(2)) - l[2].powi(2)).abs() < e
}

fn length(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
}

fn area(y1: i32, x1: i32, y2: i32, x2: i32, x3: i32, y3: i32) -> f64 {
    0.5 * ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) as f64).abs()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let triangles = generate_triangles();

    let width = 100;
    let height = 100;

    for (i, triangle) in triangles.into_iter().enumerate() {
        let file_name = format!("triangles/triangle_{}_({},{})_({},{})_({},{}).png", i, triangle[0].0, triangle[0].1, triangle[1].0, triangle[1].1, triangle[2].0, triangle[2].1);
        println!("{}", file_name);
        let root = BitMapBackend::new(&file_name, (width, height)).into_drawing_area();
        root.fill(&WHITE)?;
        draw_triangle(&root, &triangle)?;
    }

    Ok(())
}

fn draw_triangle(
    area: &DrawingArea<BitMapBackend, Shift>,
    points: &[(i32, i32)],
) -> Result<(), Box<dyn std::error::Error>> {
    let x_min = points.iter().map(|(x, _)| *x).min().unwrap();
    let x_max = points.iter().map(|(x, _)| *x).max().unwrap();
    let y_min = points.iter().map(|(_, y)| *y).min().unwrap();
    let y_max = points.iter().map(|(_, y)| *y).max().unwrap();

    let margin = 1;

    let mut chart = ChartBuilder::on(area)
        .set_label_area_size(LabelAreaPosition::Left, 0)
        .set_label_area_size(LabelAreaPosition::Right, 0)
        .set_label_area_size(LabelAreaPosition::Top, 0)
        .set_label_area_size(LabelAreaPosition::Bottom, 0)
        .build_cartesian_2d(x_min - margin..x_max + margin, y_min - margin..y_max + margin)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        points.iter().cloned().chain(repeat_with(|| points[0])).take(4),
        RED,
    ))?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let area = area(0, 0, 1, 0, 2, 2);
        println!("{}", area)
    }

    #[test]
    fn test_pythagoras() {
        assert!(!is_pythagorean(0, 0, 1, 0, 2, 2, 0.01))
    }
}