#[derive(Debug)]
pub enum CommandType {
    Ask,
    Show,
    Tell,
    Error,
}

#[derive(Debug)]
pub struct Command {
    command_type: CommandType,
    text: String,
    timer: Option<f64>,
}

impl From<String> for Command {
    fn from(command: String) -> Command {
        let mut complete_command = command.splitn(2, ": ");

        if let Some(command_type) = complete_command.next() {
            if let Some(command_contents) = complete_command.next() {
                let (command_type, text, timer) = match command_type {
                    "ask"  => {
                        (CommandType::Ask, String::from(command_contents), None)
                    },
                    "show" => {
                        let mut complete_show = command.splitn(2, " for=> ");

                        if let Some(show_contents) = complete_show.next() {
                            if let Some(show_time) = complete_show.next() {
                                if let Ok(parsed_time) = show_time.parse::<f64>() {
                                    (CommandType::Show, String::from(show_contents), Some(parsed_time))
                                } else {
                                    (CommandType::Error, format!("ERROR READING COMMAND: {}", command), None)
                                }
                            } else {
                                (CommandType::Error, format!("ERROR READING COMMAND: {}", command), None)
                            }
                        } else {
                            (CommandType::Error, format!("ERROR READING COMMAND: {}", command), None)
                        }  
                    },
                    "tell" => {
                        (CommandType::Tell, String::from(command_contents), None)   
                    },
                    _      => {
                        (CommandType::Error, format!("ERROR READING COMMAND: {}", command), None) 
                    },
                };

                return Command {
                    command_type: command_type,
                    text: text,
                    timer: timer,
                }
            }
        }

        Command {
            command_type: CommandType::Error,
            text: format!("ERROR READING COMMAND: {}", command),
            timer: None,
        }
    }
}

impl From<&str> for Command {
    fn from(command: &str) -> Command {
        Command::from(String::from(command))
    }
}
