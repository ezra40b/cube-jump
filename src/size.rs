pub struct Size {
    pub width: usize,
    pub height: usize
}

impl Size {
    pub(crate) fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }
}