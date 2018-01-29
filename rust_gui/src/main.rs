pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>
}

impl Screen {

    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing {:?}", &self)
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing {:?}", &self)
    }
}

fn main() {
    let button = Button{width: 10, height: 5, label: String::from("Button")};
    let select_box = SelectBox{width: 20, height: 5, options: vec![String::from("A"), String::from("B")]};
    let screen = Screen{components: vec![Box::new(button), Box::new(select_box)]};
    screen.run();
}
