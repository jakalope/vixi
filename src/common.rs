use std::collections::VecDeque;

pub fn move_to_front<T>(front: &mut Vec<T>, typeahead: &mut VecDeque<T>) {
    for key in front.drain(..).rev() {
        typeahead.push_front(key);
    }
}

pub fn prepend<T>(front: &Vec<T>, typeahead: &mut VecDeque<T>)
where
    T: Copy,
{
    for key in front.iter().rev() {
        typeahead.push_front(*key);
    }
}

#[cfg(test)]
mod freefunc_test {
    use super::*;

    #[test]
    fn test_move_to_front() {
        let mut front = vec![1u8, 2u8, 3u8];
        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(4u8);
        typeahead.push_back(5u8);
        move_to_front(&mut front, &mut typeahead);
        assert_eq!(0, front.len());
        assert_eq!(5, typeahead.len());
        let mut i = typeahead.iter();
        assert_eq!(Some(&1u8), i.next());
    }

    #[test]
    fn test_prepend() {
        let front = vec![1u8, 2u8, 3u8];
        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(4u8);
        typeahead.push_back(5u8);
        prepend(&front, &mut typeahead);
        assert_eq!(3, front.len());
        assert_eq!(5, typeahead.len());
        let mut i = typeahead.iter();
        assert_eq!(Some(&1u8), i.next());

    }
}
