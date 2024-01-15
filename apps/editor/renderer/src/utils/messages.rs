use std::collections::VecDeque;

pub struct RecursiveMessageQue<T> {
    que: VecDeque<T>,
}
impl<T> RecursiveMessageQue<T> {
    pub fn handle<FnType>(&mut self, mut handler: FnType)
    where
        FnType: FnMut(T) -> Option<Vec<T>>,
    {
        let mut index = 0;
        while let Some(msg) = self.que.pop_front() {
            let response = handler(msg);
            if let Some(mut msgs) = response {
                self.que.append(&mut msgs.into());
            }
        }
    }
}
impl<T> From<Vec<T>> for RecursiveMessageQue<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            que: value.into(),
        }
    }
}
impl<T> From<VecDeque<T>> for RecursiveMessageQue<T> {
    fn from(value: VecDeque<T>) -> Self {
        Self {
            que: value,
        }
    }
}
