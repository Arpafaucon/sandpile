use ansi_term::{
    Colour::{Blue, Purple, Red, Yellow, RGB},
};
use array2d::Array2D;
use clap::{App, Arg};
use raster::Color;
use std::clone::Clone;
use std::vec;

fn main() {
    let matches = App::new("Sandpile wiever")
        .version("1.0")
        .author("G. Roussel")
        .about("Compute and save sandpile pattern")
        .arg(
            Arg::with_name("output_image")
                .required(true)
                .index(1)
                .help("filename of output image"),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .value_name("INT")
                .default_value("128"),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iter")
                .takes_value(true)
                .value_name("INT")
                .default_value("1000"),
        )
        .get_matches();

    let size = matches.value_of("size").unwrap().parse::<usize>().unwrap();
    let num_iter = matches.value_of("iterations").unwrap().parse::<usize>().unwrap();
    let nr = size;
    let nc = size;
    let num_update_progress = 20;
    let mut a = array2d::Array2D::filled_with(0, nr, nc);
    let mut b = array2d::Array2D::filled_with(0, nr, nc);
    let mid_r = nr/2;
    let mid_c = nc/2;
    let k_rain_cells = vec![(mid_r, mid_c), (mid_r - 1, mid_c), (mid_r+1, mid_c)];
    a[(nr / 2, nc / 2)] = 1024;
    for i in 0..num_iter {
        if i % (num_iter / num_update_progress) == 0 {
            let progress = 100. * (i as f64 / num_iter as f64);
            println!("Computed {}%", progress);
        }
        update_sandpile(&a, &mut b, &k_rain_cells);
        a.clone_from(&b);
    }
    save_as_img(&b, matches.value_of("output_image").unwrap());
}

fn save_as_img(array: &Array2D<u16>, name: &str) {
    let n_cols = array.num_columns();
    let n_rows = array.num_rows();
    let mut img = raster::Image::blank(n_cols as i32, n_rows as i32);

    let col_0 = Color::rgb(0, 0, 0);
    let col_1 = Color::rgb(200, 0, 0);
    let col_2 = Color::rgb(200, 64, 0);
    let col_3 = Color::rgb(200, 128, 0);
    let col_4 = Color::rgb(255, 192, 0);
    let col_5 = Color::rgb(255, 255, 0);
    let col__ = Color::white();
    for i in 0..n_rows {
        for j in 0..n_cols {
            let col = match array[(i, j)] {
                0 => &col_0,
                1 => &col_1,
                2 => &col_2,
                3 => &col_3,
                4 => &col_4,
                5 => &col_5,
                _ => &col__,
            };
            img.set_pixel(j as i32, i as i32, col.clone())
                .expect("Could not set pixel");
        }
    }
    raster::save(&img, name).expect("Error when saving image");
}



fn update_grain(array: &Array2D<u16>, i: usize, j: usize, rain_cells: &[(usize, usize)]) -> u16 {
    let n_rows = array.num_rows();
    let n_cols = array.num_columns();
    let old_val = array.get(i, j).unwrap().clone();
    let remainder = if old_val >= 4 { old_val % 4 } else { old_val };
    let update_north = if i > 0 { array[(i - 1, j)] / 4 } else { 0 };
    let update_south = if i + 1 < n_rows {
        array[(i + 1, j)] / 4
    } else {
        0
    };
    let update_west = if j > 0 { array[(i, j - 1)] / 4 } else { 0 };
    let update_east = if j + 1 < n_cols {
        array[(i, j + 1)] / 4
    } else {
        0
    };
    let update_rain = if rain_cells.contains(&(i, j)) { 1 } else { 0 };
    let new_val = remainder + update_rain + update_north + update_south + update_west + update_east;
    new_val
}



fn update_sandpile(old: &Array2D<u16>, new: &mut Array2D<u16>, rain_cells: &[(usize, usize)]) {
    assert_eq!(old.column_len(), new.column_len());
    assert_eq!(old.row_len(), new.row_len());
    let n_rows = old.num_rows();
    let n_cols = old.num_columns();
    for i in 0..n_rows {
        for j in 0..n_cols {
            let new_val = update_grain(&old, i, j, rain_cells);
            new.set(i, j, new_val).unwrap();
        }
    }
}

fn format_sandpile(a: &Array2D<u16>) -> String {
    let p0: ansi_term::ANSIGenericString<'_, str> = RGB(50, 50, 50).paint("0");
    let p1: ansi_term::ANSIGenericString<'_, str> = Red.paint("1");
    let p2: ansi_term::ANSIGenericString<'_, str> = RGB(237, 127, 16).paint("2");
    let p3: ansi_term::ANSIGenericString<'_, str> = Yellow.paint("3");
    let p4: ansi_term::ANSIGenericString<'_, str> = Purple.paint("4");
    let p_: ansi_term::ANSIGenericString<'_, str> = Blue.paint("█");
    let mut out = String::from("");
    out.push_str("\x1B[2J\x1B[1;1H"); // clear screen and goto 1,1
    for i in 0..a.num_rows() {
        for j in 0..a.num_columns() {
            let val = a.get(i, j).unwrap();
            let val_str = format!("{}", val);
            let colored_val = match val {
                0 => &p0,
                1 => &p1,
                2 => &p2,
                3 => &p3,
                4 => &p4,
                _ => &p_,
            };
            out.push_str(&format!("{}", colored_val));
        }
        out.push_str("\n");
    }
    return out;
}
