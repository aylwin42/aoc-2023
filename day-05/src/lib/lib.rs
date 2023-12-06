pub mod args;
pub mod input;
pub mod parts;
pub mod almanac;

pub use input::load;
pub use parts::Error;
pub use parts::one as part1;
pub use parts::two as part2;