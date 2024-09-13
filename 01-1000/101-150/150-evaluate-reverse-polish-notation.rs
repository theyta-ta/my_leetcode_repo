impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in tokens {
            if let Ok(num) = token.parse() {
                stack.push(num);
            } else {
                // reverse order because popping from stack
                // FIFO BAYBEE !!!
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push( match &*token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!()
                } );
            }
        }

        stack.pop().unwrap()
    }
}
