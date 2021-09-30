use chrono::Duration;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum CommandActions {
    Add,
    Edit,
    Remove,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub action: CommandActions,
    pub name: String,
    pub commands: Vec<BotAction>,
}

enum CommandVariable {
    Username,
    QueueLength,
    QueuePlace,
    QueueTimeRemanining,
}

impl std::fmt::Display for CommandVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Username => write!(f, "username"),
            Self::QueueLength => write!(f, "queue_length"),
            Self::QueuePlace => write!(f, "queue_place"),
            Self::QueueTimeRemanining => write!(f, "queue_time_remaining"),
        }
    }
}

/*
 * !join
 *  join_queue;
 *  say "Welcome to the queue {username}.You are #{queue_position} out of {queue_length}."
 */

impl std::str::FromStr for CommandVariable {
    type Err = &'static str; // TODO there's a better error type out there than this

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "username" => Ok(CommandVariable::Username),
            "queue_length" => Ok(CommandVariable::QueueLength),
            "queue_place" => Ok(CommandVariable::QueuePlace),
            "queue_time_remaining" => Ok(CommandVariable::QueueTimeRemanining),
            _ => Err("Something fucky happened"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BotAction {
    Say(String),
    Wait(Duration),
}

peg::parser! {
    pub grammar action_parser() for str {
        rule seperator() = whitespace()* ";"+ whitespace()*

        rule number() -> u32 = n:$(['0'..='9']+) {? n.parse::<u32>().or(Err("u32")) }

        rule seconds(limit: u32) -> chrono::Duration = s:number() {?
            let limit = if limit == 0 || limit > 900 {
                900
            }
            else {
                limit
            };
            match s
            {
                1..=900 => Ok(chrono::Duration::seconds(s as i64)),
                _ => Err("Number must be between 1 and 900")
            }
        }

        rule string_character() -> String = "\\\"" { "\"".to_string() } / c:$([^'"' | '\\']+) { c.to_string() }

        rule string() -> String
            = ['"'] n:string_character()+ ['"'] { n.join("") }

        rule identifier() -> &'input str = ident:$([^'\"' | '!' | ';' | ' ' | '\\']+) { ident }

        rule variable() -> CommandVariable = "@@" ident:identifier() "&&" {? CommandVariable::from_str(ident) }

        rule whitespace() = [' ' | '\t' | '\n']

        pub(crate) rule say() -> BotAction
            = "say" whitespace()+ m:string() { BotAction::Say(m.to_string()) }

        pub(crate) rule wait() -> BotAction = "wait" whitespace()+ seconds:seconds(300) { BotAction::Wait(seconds) }

        rule atom() -> BotAction = c:wait() { c } / c:say() { c }

        rule command_action() -> CommandActions = "add" { CommandActions::Add }
            / "edit" { CommandActions::Edit }
            / "remove" { CommandActions::Remove }

        ///  This rule technically accepts a script on the `remove` command action. For now, the parser will parse the commands,
        ///  but this is not officially supported behavior and may be removed in the future.
        /// ```
        /// use brittlq::chatbot::actions::{action_parser, BotAction, Command, CommandActions};
        /// assert_eq!(
        ///     action_parser::command(r#"command remove rm say "Parser won't ignore me""#),
        ///     Ok(Command {
        ///         action: CommandActions::Remove,
        ///         name: "rm".to_string(),
        ///         commands: vec![BotAction::Say("Parser won't ignore me".to_string())]
        ///     })
        /// );
        /// ```
        pub rule command() -> Command = "command" whitespace()+ action:command_action()   whitespace()+
                                                               command_name:identifier() whitespace()*
                                                               commands:atom() ** seperator() {
            let name = command_name.to_string();
            Command {
                action,
                name,
                commands,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chatbot::actions::{action_parser, BotAction, Command, CommandActions};
    #[test]
    fn say() {
        assert_eq!(
            action_parser::say(r#"say "Hello, World!""#),
            Ok(BotAction::Say("Hello, World!".to_string()))
        );
        assert_eq!(
            action_parser::say(r#"say  "Hello, multiple spaces!""#),
            Ok(BotAction::Say("Hello, multiple spaces!".to_string()))
        );
        assert_eq!(
            action_parser::say("say\t\"Hello, tabs!\""),
            Ok(BotAction::Say("Hello, tabs!".to_string()))
        );

        assert_eq!(
            action_parser::say(r#"say "💩""#),
            Ok(BotAction::Say("💩".to_string()))
        );

        assert_eq!(
            action_parser::say(r#"say "\"This is a quote\"""#),
            Ok(BotAction::Say("\"This is a quote\"".to_string()))
        );

        assert!(action_parser::say(r#"say """#).is_err());
        assert!(action_parser::say(r#"say"#).is_err());
        assert!(action_parser::say(r#"say ""#).is_err());
    }

    #[test]
    fn wait() {
        assert_eq!(
            action_parser::wait("wait 1"),
            Ok(BotAction::Wait(chrono::Duration::seconds(1)))
        );
        assert_eq!(
            action_parser::wait("wait 257"),
            Ok(BotAction::Wait(chrono::Duration::seconds(257)))
        );
        assert!(action_parser::wait("wait").is_err());
        assert!(action_parser::wait("wait 901").is_err());
        assert!(action_parser::wait("wait 0").is_err());
        assert!(action_parser::wait("wait -120").is_err());
        assert!(action_parser::wait("wait 0.5").is_err());
        assert!(action_parser::wait("wait fail").is_err());
    }

    #[test]
    fn command_add() {
        let semicolon_end = r#"command add hello say "hello, world!""#;
        assert_eq!(
            action_parser::command(semicolon_end),
            Ok(Command {
                action: CommandActions::Add,
                name: "hello".to_string(),
                commands: vec![BotAction::Say("hello, world!".to_string())]
            })
        );

        let say_wait_say = r#"command add hello_wait say "hello" ; wait 1 ; say "world!""#;
        assert_eq!(
            action_parser::command(say_wait_say),
            Ok(Command {
                action: CommandActions::Add,
                name: String::from("hello_wait"),
                commands: vec![
                    BotAction::Say("hello".to_string()),
                    BotAction::Wait(chrono::Duration::seconds(1)),
                    BotAction::Say("world!".to_string())
                ]
            })
        );

        let say_semicolon = r#"command add hello_s say "hello; world""#;
        assert_eq!(
            action_parser::command(say_semicolon),
            Ok(Command {
                action: CommandActions::Add,
                name: "hello_s".to_string(),
                commands: vec![BotAction::Say("hello; world".to_string())]
            })
        );
    }

    #[test]
    fn command_edit() {
        assert_eq!(
            action_parser::command(r#"command edit foo say "bar""#),
            Ok(Command {
                action: CommandActions::Edit,
                name: "foo".to_string(),
                commands: vec![BotAction::Say("bar".to_string())]
            })
        )
    }

    #[test]
    fn command_remove() {
        assert_eq!(
            action_parser::command("command remove foo"),
            Ok(Command {
                action: CommandActions::Remove,
                name: "foo".to_string(),
                commands: vec![]
            })
        );

        assert!(action_parser::command("command remove").is_err());
    }
}