/*
It is easily proved that no equilateral triangle exists with integral length sides and integral area. However, the almost equilateral triangle 5-5-6 has an area of 12 square units.

We shall define an almost equilateral triangle to be a triangle for which two sides are equal and the third differs by no more than one unit.

Find the sum of the perimeters of all almost equilateral triangles with integral side lengths and area and whose perimeters do not exceed one billion (1,000,000,000).
*/

use rayon::prelude::*;
use std::ops::Sub;

use rug::Float;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

fn main() {
    const MAX: u64 = 1_000_000_000;
    let sum = Arc::new(AtomicU64::new(0));

    (5..=MAX / 3).into_par_iter().for_each(|a| {
        for c in &[a - 1, a + 1] {
            let b = a;
            let p: u64 = a + b + c;
            if p <= MAX && is_area_integer(a, *c) {
                println!("{} {} {}", a, b, c);
                sum.fetch_add(p, Ordering::Relaxed);
            }
        }
    });

    println!("{}", sum.load(Ordering::Relaxed));
}

fn is_area_integer(a: u64, b: u64) -> bool {
    let area = area_of_isosceles_triangle(a, b);
    let area_rounded = area.clone().round();

    let difference = area.clone().sub(area_rounded);

    let tolerance = Float::with_val(area.prec(), 1e-12);
    difference.abs() < tolerance
}

fn area_of_isosceles_triangle(a: u64, base: u64) -> Float {
    let prec = 128;
    let a_float = Float::with_val(prec, a);
    let base_float = Float::with_val(prec, base);
    let half: Float = Float::with_val(prec, 1) / 2;
    let fourth = Float::with_val(prec, 1) / 4;

    let a_float_squared: Float = a_float.clone() * a_float;
    let base_float_squared: Float = base_float.clone() * base_float;

    half * base_float_squared.clone()
        * (((a_float_squared / base_float_squared) - fourth) as Float).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_area_integer() {
        assert!(is_area_integer(5, 6));
        assert!(is_area_integer(17, 16));
        assert!(is_area_integer(65, 66));
        assert!(is_area_integer(241, 240));
        assert!(is_area_integer(901, 902));
        assert!(is_area_integer(3361, 3360));
        assert!(is_area_integer(12545, 12546));
        assert!(is_area_integer(46817, 46816));
        assert!(is_area_integer(174725, 174726));
        assert!(is_area_integer(652081, 652080));
        assert!(is_area_integer(2433601, 2433602));
        assert!(is_area_integer(9082321, 9082320));
        assert!(is_area_integer(33895685, 33895686));
        assert!(is_area_integer(126500417, 126500416));
    }

    #[test]
    fn test_area_of_isosceles_triangle() {
        assert!(area_of_isosceles_triangle(5, 6) - 12.0 < 1e-12);
        assert!(area_of_isosceles_triangle(17, 16) - 120.0 < 1e-12);
        assert!(area_of_isosceles_triangle(65, 66) - 1848.0 < 1e-12);
        assert!(area_of_isosceles_triangle(241, 240) - 25080.0 < 1e-12);
        assert!(area_of_isosceles_triangle(901, 902) - 351780.0 < 1e-12);
        assert!(area_of_isosceles_triangle(3361, 3360) - 4890480.0 < 1e-12);
        assert!(area_of_isosceles_triangle(12545, 12546) - 68149872.0 < 1e-12);
        assert!(area_of_isosceles_triangle(46817, 46816) - 949077360.0 < 1e-12);
        assert!(area_of_isosceles_triangle(174725, 174726) - 13219419708.0 < 1e-12);
        assert!(area_of_isosceles_triangle(652081, 652080) - 184120982760.0 < 1e-12);
        assert!(area_of_isosceles_triangle(2433601, 2433602) - 2564481115560.0 < 1e-12);
        assert!(area_of_isosceles_triangle(9082321, 9082320) - 35718589344360.0 < 1e-12);
        assert!(area_of_isosceles_triangle(33895685, 33895686) - 497495864091732.0 < 1e-12);
        assert!(area_of_isosceles_triangle(126500417, 126500416) - 6929223155685600.0 < 1e-12);
    }
}
