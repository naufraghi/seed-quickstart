#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

struct Model {
    pub angle: i32,
    pub iters: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self { angle: 5, iters: 12}
    }
}

// Update

#[derive(Clone)]
enum Msg {
    IncrementAngle,
    IncrementIters,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::IncrementAngle => model.angle += 1,
        Msg::IncrementIters => model.iters += 1,
    }
}

fn path(a: i32, i: i32) -> String {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;

    let axiom = String::from("A");

    let mut rules = HashMap::new();
    rules.insert('A', "AB");
    rules.insert('B', "A");

    let mut world = axiom;

    for _ in 0..i {
        world = world
            .chars()
            .map(|c| rules.get(&c).unwrap().clone())
            .collect();
        println!("{}", world);
    }

    let svg = r#"<?xml version="1.0" standalone="no"?>
    <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" 
      "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
    <svg width="100%" height="100%" viewBox="0 0 4000 4000"
      xmlns="http://www.w3.org/2000/svg" version="1.1">
      <path d="{}"
        fill="none" stroke="blue" stroke-width="3" />
    </svg>
    "#;

    #[derive(PartialEq)]
    enum Action {
        Step,
        Rotate,
        Stay,
    }

    struct State {
        act: Action,
        rot: i16,
    }

    let mut renderer = HashMap::new();
    renderer.insert('A', Action::Step);
    renderer.insert('B', Action::Rotate);

    let path: String = world
        .chars()
        .scan(
            State {
                act: Action::Stay,
                rot: 0,
            },
            |state, c| {
                use std::f32;
                let s = renderer.get(&c).unwrap();
                let rot: f32 = state.rot.into();
                let angle: f32 = rot / 3600.0 * 2.0 * f32::consts::PI;
                let x = 10.0 * angle.sin();
                let y = 10.0 * angle.cos();
                Some(match s {
                    Action::Step => {
                        //if state.act == Action::Step {
                        //    format!("l {},{} ", 10.0 * x, 10.0 * y)
                        //} else {
                        //    state.act = Action::Step;
                            format!("l {},{} ", x, y)
                        //}
                    }
                    Action::Rotate => {
                        state.rot = ((state.rot as i32 + 10 * a) % 3600) as i16;
                        state.act = Action::Rotate;
                        "".into()
                    }
                    Action::Stay => "".into(),
                })
            },
        )
        .collect();

    let d = format!("M 0,0 {}", path);

    d.into()
}

fn line(a: i32) -> String {
    format!("M 0,0 l {},{}", a, a).into()
}

// View

fn view(model: &Model) -> impl View<Msg> {
    span![
        button![
            simple_ev(Ev::Click, Msg::IncrementAngle),
            format!("Angle {}", model.angle)
        ],
        button![
            simple_ev(Ev::Click, Msg::IncrementIters),
            format!("Iters {}", model.iters)
        ],
        svg![
        attrs![At::Width => "800px", At::Height => "800px", At::ViewBox => "-40 -40 160 160"],
        path![
            attrs![At::D => path(model.angle, model.iters), At::Stroke => "blue", At::Fill => "none"]
        ]],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
