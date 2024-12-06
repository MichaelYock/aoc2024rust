pub mod template;

// Use this file to add helper functions and additional modules.

use extend::ext;

#[ext]
pub impl<T> Vec<T> {
    fn iget(&self, index: isize) -> Option<&T> {
        if index < 0 {
            None
        } else {
            self.get(index as usize)
        }
    }
}