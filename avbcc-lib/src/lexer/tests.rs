use super::*;

#[test]
fn test_return_2() {
    let text = "int main(void) { \n  return 2; \n}";

    let lexer = Lexer::new(text);
    for token in lexer {
        let token = token.unwrap();
        println!("{:?}", token);
    }
}