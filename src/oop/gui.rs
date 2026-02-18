use crate::feature::Feature;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub label: String,
    pub width: u32,
    pub height: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{} - {}/{}", self.label, self.width, self.height);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Options {:?} - {}/{}", self.options, self.width, self.height);
    }
}

pub fn start_gui(feature: &Feature) {
    if *feature != Feature::GUI {
        return;
    }

    println!("GUI start");

    let v: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {label: String::from("Click me"), width: 10, height: 10}),
        Box::new(SelectBox {width: 5, height: 5, options: vec![
            String::from("FR"), String::from("ES"), String::from("DE")]}),
    ];

    let screen = Screen {components: v};
    screen.run();
}