pub mod simple_cli;

pub trait Runnerable {
	fn new() -> Self;
	fn main_loop(&mut self);
}
