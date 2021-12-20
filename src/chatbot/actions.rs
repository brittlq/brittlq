use time::Duration;

use super::commands::{AccessLevel, Command};

#[derive(Debug, PartialEq)]
pub enum ActionTag {
    Add,
    Edit,
    Remove,
}

#[derive(Debug, PartialEq)]
pub struct EditorAction {
    pub name: String,
    pub action: ActionTag,
    pub command: Option<Command>,
}

#[derive(Debug, PartialEq)]
pub enum ScriptAction {
    Say(String),
    Wait(Duration),
}

peg::parser! {
    pub grammar action_parser() for str {
        rule seperator() = (_* ";" _*)+

        rule number() -> u32 = n:$(['0'..='9']+) {? n.parse::<u32>().or(Err("u32")) }

        rule seconds(limit: u32) -> time::Duration = s:number() {?
            let limit = if limit == 0 || limit > 900 {
                900
            }
            else {
                limit
            };
            match s
            {
                1..=900 => Ok(time::Duration::seconds(s as i64)),
                _ => Err("Number must be between 1 and 900")
            }
        }

        rule string_character() -> String = "\\\"" { "\"".to_owned() } / c:$([^'"' | '\\']+) { c.to_owned() }

        rule string() -> String
            = ['"'] n:string_character()+ ['"'] { n.join("") }

        rule identifier() -> &'input str = ident:$([^'\"' | '!' | ';' | ' ' | '\\']+) { ident }

        rule _ = [' ' | '\t' | '\n']

        rule cooldown() -> Duration = seperator() "cooldown" _+ seconds:seconds(0) { seconds }

        pub(crate) rule say() -> ScriptAction
            = "say" _+ m:string() { ScriptAction::Say(m) }

        pub(crate) rule wait() -> ScriptAction = "wait" _+ seconds:seconds(300) { ScriptAction::Wait(seconds) }

        rule atom() -> ScriptAction = c:wait() { c } / c:say() { c }

        rule add() -> EditorAction = "add" _+ command_name:identifier() _+ script:atom() ** seperator() cd:cooldown()?
        {
            EditorAction {
                name: command_name.to_owned(),
                action: ActionTag::Add,
                command: Some(Command {
                    name: command_name.to_owned(),
                    enabled: true,
                    access_level: AccessLevel::User,
                    script,
                    cooldown: cd.unwrap_or_else(|| Duration::seconds(30)),
                    last_execution: None,
                })
            }
        }

        rule edit() -> EditorAction = "edit" _+ command_name:identifier() _+ script:atom() ** seperator() cd:cooldown()?
        {
            EditorAction {
                name: command_name.to_owned(),
                action: ActionTag::Edit,
                command: Some(Command {
                    name: command_name.to_owned(),
                    enabled: true,
                    access_level: AccessLevel::User,
                    script,
                    cooldown: cd.unwrap_or_else(|| Duration::seconds(30)),
                    last_execution: None,
                })
            }
        }

        pub rule command() -> EditorAction = "command" _+ "remove" _+ ident:identifier() {
            EditorAction {
                name: ident.to_owned(),
                action: ActionTag::Remove,
                command: None,
            }
        }
        / "command" _+ action:add() { action }
        / "command" _+ action:edit() { action }
    }
}

#[cfg(test)]
mod tests {
    use crate::chatbot::{
        actions::{action_parser, ActionTag, EditorAction, ScriptAction},
        commands::{AccessLevel, Command, ExecutionError},
    };
    use irc::client::prelude::Config;
    use time::Duration;

    fn test_config() -> irc::client::prelude::Config {
        irc::client::prelude::Config {
            owners: vec![format!("test")],
            nickname: Some(format!("test")),
            server: Some(format!("irc.test.net")),
            channels: vec![format!("#test")],
            user_info: Some(format!("Testing.")),
            use_mock_connection: true,
            ..Default::default()
        }
    }

    #[test]
    fn say() {
        assert_eq!(
            action_parser::say(r#"say "Hello, World!""#),
            Ok(ScriptAction::Say("Hello, World!".to_owned()))
        );
        assert_eq!(
            action_parser::say(r#"say  "Hello, multiple spaces!""#),
            Ok(ScriptAction::Say("Hello, multiple spaces!".to_owned()))
        );
        assert_eq!(
            action_parser::say("say\t\"Hello, tabs!\""),
            Ok(ScriptAction::Say("Hello, tabs!".to_owned()))
        );

        assert_eq!(
            action_parser::say(r#"say "💩""#),
            Ok(ScriptAction::Say("💩".to_owned()))
        );

        assert_eq!(
            action_parser::say(r#"say "\"This is a quote\"""#),
            Ok(ScriptAction::Say("\"This is a quote\"".to_owned()))
        );

        assert!(action_parser::say(r#"say """#).is_err());
        assert!(action_parser::say(r#"say"#).is_err());
        assert!(action_parser::say(r#"say ""#).is_err());
    }

    #[test]
    fn wait() {
        assert_eq!(
            action_parser::wait("wait 1"),
            Ok(ScriptAction::Wait(time::Duration::seconds(1)))
        );
        assert_eq!(
            action_parser::wait("wait 257"),
            Ok(ScriptAction::Wait(time::Duration::seconds(257)))
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
            Ok(EditorAction {
                name: "hello".to_owned(),
                action: ActionTag::Add,
                command: Some(Command {
                    name: "hello".to_owned(),
                    enabled: true,
                    access_level: AccessLevel::User,
                    script: vec![ScriptAction::Say("hello, world!".to_owned())],
                    cooldown: Duration::seconds(30),
                    last_execution: None
                })
            })
        );

        let say_wait_say = r#"command add hello_wait say "hello" ; wait 1 ; say "world!""#;
        assert_eq!(
            action_parser::command(say_wait_say),
            Ok(EditorAction {
                name: "hello_wait".to_owned(),
                action: ActionTag::Add,
                command: Some(Command {
                    name: String::from("hello_wait"),
                    enabled: true,
                    access_level: AccessLevel::User,
                    script: vec![
                        ScriptAction::Say("hello".to_owned()),
                        ScriptAction::Wait(time::Duration::seconds(1)),
                        ScriptAction::Say("world!".to_owned())
                    ],
                    cooldown: Duration::seconds(30),
                    last_execution: None
                })
            })
        );

        let say_semicolon = r#"command add hello_s say "hello; world""#;
        assert_eq!(
            action_parser::command(say_semicolon),
            Ok(EditorAction {
                name: "hello_s".to_owned(),
                action: ActionTag::Add,
                command: Some(Command {
                    name: "hello_s".to_owned(),
                    enabled: true,
                    access_level: AccessLevel::User,
                    script: vec![ScriptAction::Say("hello; world".to_owned())],
                    cooldown: Duration::seconds(30),
                    last_execution: None
                })
            })
        );
    }

    #[test]
    fn command_edit() {
        assert_eq!(
            action_parser::command(r#"command edit foo say "bar""#),
            Ok(EditorAction {
                name: "foo".to_owned(),
                action: ActionTag::Edit,
                command: Some(Command {
                    name: "foo".to_owned(),
                    enabled: true,
                    access_level: AccessLevel::User,
                    script: vec![ScriptAction::Say("bar".to_owned())],
                    cooldown: Duration::seconds(30),
                    last_execution: None
                })
            })
        );
    }

    #[test]
    fn command_remove() {
        assert_eq!(
            action_parser::command("command remove foo"),
            Ok(EditorAction {
                name: "foo".to_owned(),
                action: ActionTag::Remove,
                command: None,
            })
        );

        assert!(action_parser::command("command remove").is_err());
    }

    #[tokio::test]
    async fn command_cooldown() {
        let mut test =
            action_parser::command(r#"command add hello say "hello"; cooldown 60"#).unwrap();
        assert_eq!(
            test,
            EditorAction {
                name: "hello".to_owned(),
                action: ActionTag::Add,
                command: Some(Command {
                    name: "hello".to_owned(),
                    enabled: true,
                    access_level: AccessLevel::User,
                    script: vec![ScriptAction::Say("hello".to_owned())],
                    cooldown: Duration::seconds(60),
                    last_execution: None
                })
            }
        );
        let client = irc::client::Client::from_config(test_config())
            .await
            .unwrap();
        let sender = client.sender();

        let mut test = test.command.unwrap();
        assert!(test.execute("#test", &sender).await.is_ok());
        let cooldown_err = test.execute("#test", &sender).await;
        assert!(matches!(cooldown_err, Err(ExecutionError::Cooldown(..))));
    }
}
