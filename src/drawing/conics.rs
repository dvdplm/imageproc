use image::{GenericImage, ImageBuffer};
use definitions::Image;
use std::f32;
use std::i32;
use drawing::draw_if_in_bounds;
use drawing::line::draw_line_segment_mut;

/// Draw as much of an ellipse as lies inside the image bounds.
/// Uses Midpoint Ellipse Drawing Algorithm. (Modified from Bresenham's algorithm) (http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)
///
/// The ellipse is axis-aligned and satisfies the following equation:
///
/// (`x^2 / width_radius^2) + (y^2 / height_radius^2) = 1`
pub fn draw_hollow_ellipse<I>(
    image: &I,
    center: (i32, i32),
    width_radius: i32,
    height_radius: i32,
    color: I::Pixel,
) -> Image<I::Pixel>
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0);
    draw_hollow_ellipse_mut(&mut out, center, width_radius, height_radius, color);
    out
}

/// Draw as much of an ellipse as lies inside the image bounds.
/// Uses Midpoint Ellipse Drawing Algorithm. (Modified from Bresenham's algorithm) (http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)
///
/// The ellipse is axis-aligned and satisfies the following equation:
///
/// `(x^2 / width_radius^2) + (y^2 / height_radius^2) = 1`
pub fn draw_hollow_ellipse_mut<I>(
    image: &mut I,
    center: (i32, i32),
    width_radius: i32,
    height_radius: i32,
    color: I::Pixel,
) where
    I: GenericImage,
    I::Pixel: 'static,
{
    // Circle drawing algorithm is faster, so use it if the given ellipse is actually a circle.
    if width_radius == height_radius {
        draw_hollow_circle_mut(image, center, width_radius, color);
        return;
    }

    let draw_quad_pixels = |x0: i32, y0: i32, x: i32, y: i32| {
        draw_if_in_bounds(image, x0 + x, y0 + y, color);
        draw_if_in_bounds(image, x0 - x, y0 + y, color);
        draw_if_in_bounds(image, x0 + x, y0 - y, color);
        draw_if_in_bounds(image, x0 - x, y0 - y, color);
    };

    draw_ellipse(draw_quad_pixels, center, width_radius, height_radius);
}

/// Draw as much of an ellipse, including its contents, as lies inside the image bounds.
/// Uses Midpoint Ellipse Drawing Algorithm. (Modified from Bresenham's algorithm) (http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)
///
/// The ellipse is axis-aligned and satisfies the following equation:
///
/// `(x^2 / width_radius^2) + (y^2 / height_radius^2) <= 1`
pub fn draw_filled_ellipse<I>(
    image: &I,
    center: (i32, i32),
    width_radius: i32,
    height_radius: i32,
    color: I::Pixel,
) -> Image<I::Pixel>
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0);
    draw_filled_ellipse_mut(&mut out, center, width_radius, height_radius, color);
    out
}

/// Draw as much of an ellipse, including its contents, as lies inside the image bounds.
/// Uses Midpoint Ellipse Drawing Algorithm. (Modified from Bresenham's algorithm) (http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)
///
/// The ellipse is axis-aligned and satisfies the following equation:
///
/// `(x^2 / width_radius^2) + (y^2 / height_radius^2) <= 1`
pub fn draw_filled_ellipse_mut<I>(
    image: &mut I,
    center: (i32, i32),
    width_radius: i32,
    height_radius: i32,
    color: I::Pixel,
) where
    I: GenericImage,
    I::Pixel: 'static,
{
    // Circle drawing algorithm is faster, so use it if the given ellipse is actually a circle.
    if width_radius == height_radius {
        draw_filled_circle_mut(image, center, width_radius, color);
        return;
    }

    let draw_line_pairs = |x0: i32, y0: i32, x: i32, y: i32| {
        draw_line_segment_mut(
            image,
            ((x0 - x) as f32, (y0 + y) as f32),
            ((x0 + x) as f32, (y0 + y) as f32),
            color,
        );
        draw_line_segment_mut(
            image,
            ((x0 - x) as f32, (y0 - y) as f32),
            ((x0 + x) as f32, (y0 - y) as f32),
            color,
        );
    };

    draw_ellipse(draw_line_pairs, center, width_radius, height_radius);
}

// Implements the Midpoint Ellipse Drawing Algorithm. (Modified from Bresenham's algorithm) (http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)
// Takes a function that determines how to render the points on the ellipse.
fn draw_ellipse<F>(mut render_func: F, center: (i32, i32), width_radius: i32, height_radius: i32)
where
    F: FnMut(i32, i32, i32, i32),
{
    let (x0, y0) = center;
    let w2 = width_radius * width_radius;
    let h2 = height_radius * height_radius;
    let mut x = 0;
    let mut y = height_radius;
    let mut px = 0;
    let mut py = 2 * w2 * y;

    render_func(x0, y0, x, y);

    // Top and bottom regions.
    let mut p = (h2 - (w2 * height_radius)) as f32 + (0.25 * w2 as f32);
    while px < py {
        x += 1;
        px += 2 * h2;
        if p < 0.0 {
            p += (h2 + px) as f32;
        } else {
            y -= 1;
            py += -2 * w2;
            p += (h2 + px - py) as f32;
        }

        render_func(x0, y0, x, y);
    }

    // Left and right regions.
    p = (h2 as f32) * (x as f32 + 0.5).powi(2) + (w2 * (y - 1).pow(2)) as f32 - (w2 * h2) as f32;
    while y > 0 {
        y -= 1;
        py += -2 * w2;
        if p > 0.0 {
            p += (w2 - py) as f32;
        } else {
            x += 1;
            px += 2 * h2;
            p += (w2 - py + px) as f32;
        }

        render_func(x0, y0, x, y);
    }
}

/// Draw as much of a circle as lies inside the image bounds.
pub fn draw_hollow_circle<I>(
    image: &I,
    center: (i32, i32),
    radius: i32,
    color: I::Pixel,
) -> Image<I::Pixel>
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0);
    draw_hollow_circle_mut(&mut out, center, radius, color);
    out
}

/// Draw as much of a circle as lies inside the image bounds.
pub fn draw_hollow_circle_mut<I>(image: &mut I, center: (i32, i32), radius: i32, color: I::Pixel)
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut x = radius;
    let mut y = 0i32;
    let mut err = 0i32;
    let x0 = center.0;
    let y0 = center.1;

    while x >= y {
        draw_if_in_bounds(image, x0 + x, y0 + y, color);
        draw_if_in_bounds(image, x0 + y, y0 + x, color);
        draw_if_in_bounds(image, x0 - y, y0 + x, color);
        draw_if_in_bounds(image, x0 - x, y0 + y, color);
        draw_if_in_bounds(image, x0 - x, y0 - y, color);
        draw_if_in_bounds(image, x0 - y, y0 - x, color);
        draw_if_in_bounds(image, x0 + y, y0 - x, color);
        draw_if_in_bounds(image, x0 + x, y0 - y, color);

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
}

/// Draw as much of a circle, including its contents, as lies inside the image bounds.
pub fn draw_filled_circle_mut<I>(image: &mut I, center: (i32, i32), radius: i32, color: I::Pixel)
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut x = radius;
    let mut y = 0i32;
    let mut err = 0i32;
    let x0 = center.0;
    let y0 = center.1;

    while x >= y {
        draw_line_segment_mut(
            image,
            ((x0 - x) as f32, (y0 + y) as f32),
            ((x0 + x) as f32, (y0 + y) as f32),
            color,
        );
        draw_line_segment_mut(
            image,
            ((x0 - y) as f32, (y0 + x) as f32),
            ((x0 + y) as f32, (y0 + x) as f32),
            color,
        );
        draw_line_segment_mut(
            image,
            ((x0 - x) as f32, (y0 - y) as f32),
            ((x0 + x) as f32, (y0 - y) as f32),
            color,
        );
        draw_line_segment_mut(
            image,
            ((x0 - y) as f32, (y0 - x) as f32),
            ((x0 + y) as f32, (y0 - x) as f32),
            color,
        );

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
}

/// Draw as much of a circle and its contents as lies inside the image bounds.
pub fn draw_filled_circle<I>(
    image: &I,
    center: (i32, i32),
    radius: i32,
    color: I::Pixel,
) -> Image<I::Pixel>
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0);
    draw_filled_circle_mut(&mut out, center, radius, color);
    out
}

#[cfg(test)]
mod test {
    use super::*;
    use image::{GrayImage, Luma};
    use test::{Bencher, black_box};

    macro_rules! bench_hollow_ellipse {
        ($name:ident, $center:expr, $width_radius:expr, $height_radius:expr) => {
            #[bench]
            fn $name(b: &mut test::Bencher) {
                use super::draw_hollow_ellipse_mut;

                let mut image = GrayImage::new(500, 500);
                let color = Luma([50u8]);
                b.iter(|| {
                    draw_hollow_ellipse_mut(&mut image, $center, $width_radius, $height_radius, color);
                    test::black_box(&image);
                    });
            }
        }
    }

    bench_hollow_ellipse!(bench_bench_hollow_ellipse_circle, (200, 200), 80, 80);
    bench_hollow_ellipse!(bench_bench_hollow_ellipse_vertical, (200, 200), 40, 100);
    bench_hollow_ellipse!(bench_bench_hollow_ellipse_horizontal, (200, 200), 100, 40);

    macro_rules! bench_filled_ellipse {
        ($name:ident, $center:expr, $width_radius:expr, $height_radius:expr) => {
            #[bench]
            fn $name(b: &mut test::Bencher) {
                use super::draw_filled_ellipse_mut;

                let mut image = GrayImage::new(500, 500);
                let color = Luma([50u8]);
                b.iter(|| {
                    draw_filled_ellipse_mut(&mut image, $center, $width_radius, $height_radius, color);
                    test::black_box(&image);
                    });
            }
        }
    }

    bench_filled_ellipse!(bench_bench_filled_ellipse_circle, (200, 200), 80, 80);
    bench_filled_ellipse!(bench_bench_filled_ellipse_vertical, (200, 200), 40, 100);
    bench_filled_ellipse!(bench_bench_filled_ellipse_horizontal, (200, 200), 100, 40);
}
