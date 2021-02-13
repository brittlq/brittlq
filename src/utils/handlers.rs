use crate::utils::bot::Args;

pub fn peek(args: Args) {
    let popped_entries: Vec<_> = (&args
        .queue
        .queue)
        .into_iter()
        .take(4)
        .map(|x| x.nickname.clone())
        .collect();
    if !popped_entries.is_empty() {
        args.writer
            .send_privmsg(args.msg.target, &popped_entries.join(", "))
            .unwrap();
    } else {
        args.writer
            .send_privmsg(args.msg.target, "The queue is empty")
            .unwrap();
    }
}

