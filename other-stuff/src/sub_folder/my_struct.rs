pub struct MyStruct {}
pub enum Another {}

trait Bar {
    fn do_stuff(&self);
}

impl Bar for MyStruct {
    fn do_stuff(&self) {
        self.do_stuff();
    }
}

impl Bar for Another {
    fn do_stuff(&self) {
        self.do_stuff();
    }
}

impl std::default::Default for MyStruct {
    fn default() -> Self {
        MyStruct { }
    }
}