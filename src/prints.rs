use colored::*;

pub fn prompt(text: &str) {
  println!(
    "{} {}",
    colored::ColoredString::from("prmt* ").color("magenta"),
    colored::ColoredString::from(text).color("magenta")
  );
}

pub fn error(text: &str) {
  println!(
    "{} {}",
    colored::ColoredString::from("errr* ").color("red"),
    colored::ColoredString::from(text).color("red")
  );
}

pub fn info(text: &str) {
  println!(
    "{} {}",
    colored::ColoredString::from("info* ").color("cyan"),
    colored::ColoredString::from(text).color("cyan")
  );
}

pub fn warning(text: &str) {
  println!(
    "{} {}",
    colored::ColoredString::from("warn* ").color("yellow"),
    colored::ColoredString::from(text).color("yellow")
  );
}

