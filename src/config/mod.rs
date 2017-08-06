/// Configuration struct for parsing command line inputs
pub struct Config {
    /// A .ray file as the scene input
    pub ray_filename: String,
    /// An output filename for our rendered scene
    pub output_filename: String,
}

impl Config {
    /// Construction for Config struct
    ///
    /// # arguements
    ///
    /// * `args` - a reference to an array of strings representing the command line arguements
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguements");
        }

        let ray_filename = args[1].clone();
        let output_filename = args[2].clone();

        Ok(Config { ray_filename, output_filename })
    }
}