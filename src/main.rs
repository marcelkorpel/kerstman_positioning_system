use plotters::prelude::*;
use std::fs;

fn main() {
    let path = "4F88BQHUA47M.txt";
    let mut richting = 0;
    let mut x = 0;
    let mut y = 0;
    let mut coord: Vec<(i32, i32)> = vec![];

    let drawing_area =
        BitMapBackend::new("plot_without_lines.png", (1000, 100)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..100, 0..10)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();

    let input = fs::read_to_string(path).expect("Navigatie-instructies niet gevonden!");
    let lines = input.lines();

    for line in lines {
        let mut line = line.split_whitespace();
        let command = line.next().unwrap();

        match command {
            "draai" => {
                richting += line.next().unwrap().parse::<i32>().unwrap();
            }
            &_ => {
                let (move_x, move_y) = match richting % 360 {
                    0 => (0, 1),
                    45 => (1, 1),
                    90 => (1, 0),
                    135 => (1, -1),
                    180 => (0, -1),
                    225 => (-1, -1),
                    270 => (-1, 0),
                    315 => (-1, 1),
                    _ => panic!("De kerstman overtreedt de geometrische regels!"),
                };

                let afstand = line.next().unwrap().parse::<i32>().unwrap();
                x += move_x * afstand;
                y += move_y * afstand;
                coord.push((x, y));
                println!("{} naar {}, {}", command, x, y);
            }
        }
    }

    let afstand = x + y;
    println!("Afstand: {}", afstand);

    ctx.draw_series(coord.iter().map(|point| Circle::new(*point, 3, &BLUE)))
        .unwrap();
}
