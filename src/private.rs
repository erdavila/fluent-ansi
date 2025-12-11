use crate::Format;

pub trait ModifyFormat {
    fn modify_format(self, modify: impl Fn(Format) -> Format) -> Self;
}
