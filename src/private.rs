use crate::Format;

pub trait PrivateWithFormat: Sized {
    fn modify_format(self, m: impl Fn(&mut Format)) -> Self {
        let mut format = self.get_format();
        m(&mut format);
        self.with_format(format)
    }

    fn get_format(&self) -> Format;

    fn with_format(self, format: Format) -> Self;
}
