use crate::prelude::*;
use svg::node::element::path::{Command, Data, Position};
use svg::parser::Event;

pub fn parse_path(app: &App, svg_file_name: &str) -> Vec<Point2> {
    let path_to_svg_file = app.assets_path().unwrap().join(svg_file_name);

    let mut content = String::new();

    let mut points: Vec<Point2> = vec![];

    let mut current_point = pt2(0.0f32, 0.0f32);

    for event in svg::open(path_to_svg_file, &mut content).unwrap() {
        if let Event::Tag(_path, _, attributes) = event {
            let data = attributes.get("d");
            if data.is_none() {
                continue;
            }
            let data = data.unwrap();

            let data = Data::parse(data).unwrap();
            for command in data.iter() {
                match command {
                    Command::Move(position, params) => {
                        // println!("Move!");
                        if let Position::Relative = position {
                            panic!("Our SVG parser doesn't handle relative commands.");
                        }

                        let x = params.get(0).unwrap();
                        let y = params.get(1).unwrap();

                        let point = pt2(*x, *y);
                        current_point = point;

                        points.push(point);
                    }
                    Command::Line(position, params) => {
                        if let Position::Relative = position {
                            panic!("Our SVG parser doesn't handle relative commands.");
                        }

                        let x = params.get(0).unwrap();
                        let y = params.get(1).unwrap();

                        let point = pt2(*x, *y);
                        current_point = point;

                        points.push(point);
                    }
                    Command::HorizontalLine(position, params) => {
                        // if let Position::Relative = position {
                        //     panic!("Our SVG parser doesn't handle relative commands.");
                        // }
                        // println!("Horizontal line");

                        // let x = params.get(0).unwrap();

                        // let point = pt2(*x, 0.0) + current_point;
                        // current_point = point;

                        // points.push(point);
                    }
                    Command::VerticalLine(position, params) => {
                        // if let Position::Relative = position {
                        //     panic!("Our SVG parser doesn't handle relative commands.");
                        // }
                        // println!("Vertical line");

                        // let y = params.get(0).unwrap();

                        // let point = pt2(0.0, *y) + current_point;
                        // current_point = point;

                        // points.push(point);
                    }
                    _ => {}
                }
            }
        }
    }

    points.normalize().invert_normalized_y()
}
