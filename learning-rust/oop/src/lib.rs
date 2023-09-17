
// this is just an example not use anywhere
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f32,
}

// encapsulation example
impl AveragedCollection {

    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) ->Option<i32>{
        let result = self.list.pop();
        match result {
            Some(value) =>{
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f32 {
        self.average
    }
    
    fn update_average(&mut self){
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f32 / self.list.len() as f32;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen{
    //this is more costly than generic since it has a vtable
    pub components: Vec<Box<dyn Draw>>,
}

/// different from a generic implementation because we can only
/// have one type!
/// # example
/// pub struct Screen<T: Draw> {
///     pub comps:Vec<T>,
/// }
/// impl<T> Screen<T> {
/// where
/// T:Draw
/// {
/// pub fn run(&self){
/// for comp in self.components.iter() {
///     comp.draw();
/// }
/// }
/// 
/// }


impl Screen {
    pub fn run(&self){
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        
    }
}

