mod parser;
mod generator;
mod config;

fn main() {
  match config::read_config() {
    Some(config) => {
      let mut paths: Vec<String> = vec![];
      for pattern in config.files_patterns {
        paths.push(format!("{}/{}", config.project_path, pattern));
      }

      let articles = parser::parse_path(paths);

      generator::generate_docs(articles, config.docs_folder)
    },
    None => println!("Cannot find the config file"),
  }
}

