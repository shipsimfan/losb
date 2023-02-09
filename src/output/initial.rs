#[derive(PartialEq, Eq)]
#[allow(unused)]
pub enum Initial {
    None,
    NewLine,
    NewLineNotFirst,
    NewLineIfFirst,
}

impl Initial {
    pub(super) fn execute(self, first: bool) {
        match self {
            Initial::None => {}
            Initial::NewLine => println!(),
            Initial::NewLineNotFirst | Initial::NewLineIfFirst => {
                match first == (Initial::NewLineIfFirst == self) {
                    true => println!(),
                    false => {}
                }
            }
        }
    }
}
