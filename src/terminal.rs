pub mod interface {
    use core::fmt;
    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
        fn flush(&self);
    }

    pub trait Read {
        fn read_char(&self) -> char {
            ' '
        }
        fn clear(&self);
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize { 0 }
        fn chars_read(&self) -> usize { 0 }
    }

    pub trait All = Write + Read + Statistics;
}
