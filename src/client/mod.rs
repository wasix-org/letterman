mod build;
mod execute;
mod parse;
mod resolve;
pub use build::Email;
pub mod nsync;

#[derive(Debug,Clone)]
pub struct Action {
    pub tag:&'static str,
    pub cate:&'static str,
    pub io:&'static str,
    pub cmd:String
}
