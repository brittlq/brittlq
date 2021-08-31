#[derive(Debug, PartialEq)]
pub enum BotAction {
    Say(String),
}

pub enum ChatCommand {
    AddCommand(String),
    RemoveCommand(String),
}

peg::parser! {
    grammar action_parser() for str {
        // rule terminal() = semicolon:';'
        rule string_character() -> String = "\\\"" { "\"".to_string() } / c:$([^'"' | '\\']+) { c.to_string() }

        rule string() -> String
            = ['"'] n:string_character()+ ['"'] { n.join("") }

        rule whitespace() = [' ' | '\t' | '\n']*

        pub rule say() -> BotAction
            = "say" whitespace() m:string() { BotAction::Say(m.to_string()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::chatbot::actions::{action_parser, BotAction};
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
    }
}
