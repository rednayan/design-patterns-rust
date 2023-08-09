// https://web.archive.org/web/20201112023149/https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// building interfaces which accepts string
// solving the problem of always using "to_string()"  which is analagous of using ".clone()" to get around with the borrow checker
// DEREF COERCION

fn print_me(msg: &str) {
    println!("{msg}");
}

pub fn coercion() {
    let owned_string = "hello world".to_string();
    print_me(&owned_string);

    let counted_string = std::rc::Rc::new("hello world".to_string());
    print_me(&counted_string);
    let atomically_counted_string = std::sync::Arc::new("hello world".to_string());
    print_me(&atomically_counted_string);
}

// for structs its a little different because of lifetimes so need to consider diffrent scenerios

// one way to  with lifetime specifier
#[derive(Debug)]
pub struct P<'a> {
    pub n: &'a str,
}

impl<'a> P<'a> {
    pub fn g(&self) {
        println!("{}", self.n);
    }
}

// Do I need to use the variable outside of my struct? Here is a contrived example:
// Is my type large? If the type is large, then passing it by reference will save unncessary memory usage. Remember, passing by reference does not cause a copy of the variable. Consider a String buffer that contains a large amount of data. Copying that around will cause the program to be much slower.

#[derive(Debug)]
pub struct F {
    pub n: String,
}

impl F {
    pub fn g(&self) {
        println!("{}", self.n);
    }
}
