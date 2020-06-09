use super::PtraceSender;
use rustyline::Editor;

pub struct CommandHandler<'a> {
    editor: Editor<()>,
    sender: &'a PtraceSender,
}

impl<'a> CommandHandler<'a> {
    pub fn new(sender: &'a PtraceSender) -> Self {
        CommandHandler {
            editor: Editor::<()>::new(),
            sender,
        }
    }
}
