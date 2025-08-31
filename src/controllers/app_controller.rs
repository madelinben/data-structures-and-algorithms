use crate::prelude::*;
use crate::models::{AppConfig, MainMenuChoice};
use crate::views::{MenuDisplay, ConsoleView};
use crate::controllers::{SearchController, SortController};
use clap::{Command, Arg, ArgMatches};

pub struct AppController {
    config: AppConfig,
    console: ConsoleView,
    menu_display: MenuDisplay,
    search_controller: SearchController,
    sort_controller: SortController,
}

impl AppController {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
            console: ConsoleView::new(),
            menu_display: MenuDisplay::new(),
            search_controller: SearchController::new(),
            sort_controller: SortController::new(),
        }
    }
    
    pub async fn run(&mut self) -> Result<()> {
        let matches = self.create_cli().get_matches();
        
        match matches.subcommand() {
            Some(("search", sub_matches)) => {
                self.handle_search_command(sub_matches).await?;
            }
            Some(("sort", sub_matches)) => {
                self.handle_sort_command(sub_matches).await?;
            }
            _ => {
                self.run_interactive_mode().await?;
            }
        }
        
        Ok(())
    }
    
    async fn run_interactive_mode(&mut self) -> Result<()> {
        loop {
            match self.menu_display.show_main_menu()? {
                MainMenuChoice::Search => {
                    self.search_controller.run_interactive().await?;
                }
                MainMenuChoice::Sort => {
                    self.sort_controller.run_interactive().await?;
                }
                MainMenuChoice::Quit => {
                    self.console.print_goodbye();
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_search_command(&mut self, matches: &ArgMatches) -> Result<()> {
        let words_file = matches.get_one::<String>("words")
            .ok_or_else(|| Error::input("Words file not specified"))?;
        
        let iterations: usize = matches.get_one::<String>("iterations")
            .ok_or_else(|| Error::input("Iterations not specified"))?
            .parse()
            .map_err(|_| Error::validation("Invalid iterations number"))?;
        
        let target_word = matches.get_one::<String>("target").cloned();
        
        self.search_controller.run_cli(words_file, target_word, iterations).await
    }
    
    async fn handle_sort_command(&mut self, matches: &ArgMatches) -> Result<()> {
        let size: usize = matches.get_one::<String>("size")
            .ok_or_else(|| Error::input("Size not specified"))?
            .parse()
            .map_err(|_| Error::validation("Invalid size number"))?;
        
        let iterations: usize = matches.get_one::<String>("iterations")
            .ok_or_else(|| Error::input("Iterations not specified"))?
            .parse()
            .map_err(|_| Error::validation("Invalid iterations number"))?;
        
        let gui_enabled = matches.get_flag("gui");
        
        self.sort_controller.run_cli(size, iterations, gui_enabled).await
    }
    
    fn create_cli(&self) -> Command {
        Command::new("Data Structures and Algorithms")
            .version("0.1.0")
            .about("A Rust project for exploring data structures and algorithms")
            .subcommand(
                Command::new("search")
                    .about("Search Algorithm Benchmarking System")
                    .arg(
                        Arg::new("words")
                            .short('w')
                            .long("words")
                            .value_name("FILE")
                            .help("Path to words file for search benchmarking")
                            .default_value("data/words.txt")
                    )
                    .arg(
                        Arg::new("target")
                            .short('t')
                            .long("target")
                            .value_name("WORD")
                            .help("Target word to search for")
                    )
                    .arg(
                        Arg::new("iterations")
                            .short('i')
                            .long("iterations")
                            .value_name("NUM")
                            .help("Number of iterations for benchmarking")
                            .default_value("100")
                    )
            )
            .subcommand(
                Command::new("sort")
                    .about("Sorting Algorithm Benchmarking System")
                    .arg(
                        Arg::new("size")
                            .short('s')
                            .long("size")
                            .value_name("SIZE")
                            .help("Array size for sorting benchmarks")
                            .default_value("1000")
                    )
                    .arg(
                        Arg::new("iterations")
                            .short('i')
                            .long("iterations")
                            .value_name("NUM")
                            .help("Number of iterations for benchmarking")
                            .default_value("10")
                    )
                    .arg(
                        Arg::new("gui")
                            .long("gui")
                            .help("Enable GUI visualisation")
                            .action(clap::ArgAction::SetTrue)
                    )
            )
    }
}

impl Default for AppController {
    fn default() -> Self {
        Self::new()
    }
}
