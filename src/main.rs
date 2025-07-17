use untwine::{parser, parser_repl};

parser! {
    [recover = true]
    sep = #{char::is_ascii_whitespace}*;
    pub expression = or_expression -> ();
    or_expression = (and_expression sep "||" sep and_expression | and_expression ) -> ();
    and_expression = (comparison_expression sep "&&" sep comparison_expression | comparison_expression ) -> ();
    comparison_expression = (bitwise_or_expression sep ("==" | "!=" | "<" | ">" | "<=" | ">=") sep bitwise_or_expression | bitwise_or_expression ) -> ();
    bitwise_or_expression = (bitwise_xor_expression sep "|" sep bitwise_xor_expression | bitwise_xor_expression) -> ();
    bitwise_xor_expression = (bitwise_and_expression sep "^" sep bitwise_and_expression | bitwise_and_expression) -> ();
    bitwise_and_expression = (bitwise_shift_expression sep "&" sep bitwise_shift_expression | bitwise_shift_expression) -> ();
    bitwise_shift_expression = (additive_expression sep ("<<" | ">>") sep additive_expression | additive_expression) -> ();
    additive_expression = (multiplicative_expression sep ("+" | "-") sep multiplicative_expression | multiplicative_expression) -> ();
    multiplicative_expression = (unary_expression sep ("*" | "/" | "%") sep unary_expression | unary_expression) -> ();
    unary_expression = ((("-" | "!") unary_expression) | primary_expression) -> ();
    primary_expression = ("(" sep expression sep ")" | literal | identifier) -> ();
    literal = #["0123456789"]+ -> ();
    identifier = #["ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_"]+ -> ();
}

fn main() {
    parser_repl(expression);
}
