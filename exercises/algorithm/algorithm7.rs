#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            self.data.pop()
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    let pairs = [('(', ')'), ('{', '}'), ('[', ']')];

    for ch in bracket.chars() {
        if pairs.iter().any(|&(open, _)| open == ch) {
            stack.push(ch);
        } else if pairs.iter().any(|&(_, close)| close == ch) {
            if let Some(last) = stack.pop() {
                if !pairs.iter().any(|&(open, close)| open == last && close == ch) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
