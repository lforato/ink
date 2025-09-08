use crate::widgets::message::OFFSET;

pub fn get_height(text: &String) -> u16 {
    let count = text.split('\n').count();
    (count + OFFSET).max(3) as u16
}

pub fn get_longest_string(text: &String) -> usize {
    let mut longest: usize = 0;

    text.clone().split('\n').into_iter().for_each(|item| {
        if item.len() > longest as usize {
            longest = item.len() as usize;
        }
    });

    return longest;
}
