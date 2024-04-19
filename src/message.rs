pub trait Command {
    fn get_command(&self) -> String;
}

pub struct StartCommand {
    pub command: String
}

impl StartCommand {
    pub fn new() -> Self {
        StartCommand { command: "start".to_string() }
    }
}

impl Command for StartCommand {
    fn get_command(&self) -> String {
        return self.command.clone();
    }
}

pub struct StopCommand {
    pub command: String
}

impl StopCommand {
    pub fn new() -> Self {
        StopCommand { command: "stop".to_string() }
    }
}

impl Command for StopCommand {
    fn get_command(&self) -> String {
        return self.command.clone();
    }
}

pub struct InfoCommand {
    pub command: String
}

impl InfoCommand {
    pub fn new() -> Self {
        InfoCommand { command: "info".to_string() }
    }
}

impl Command for InfoCommand {
    fn get_command(&self) -> String {
        return self.command.clone();
    }
}

