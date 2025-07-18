use untwine::{parser, parser_repl};

parser! {
    [recover = true]
    __ = #{char::is_ascii_whitespace}*;
    pub expression = or_expression -> ();
    or_expression = and_expression (__ "||" __ and_expression)* -> ();
    and_expression = comparison_expression (__ "&&" __ comparison_expression)* -> ();
    comparison_expression = bitwise_or_expression (__ ("==" | "!=" | "<" | ">" | "<=" | ">=") __ bitwise_or_expression)? -> ();
    bitwise_or_expression = bitwise_xor_expression (__ "|" __ bitwise_xor_expression)* -> ();
    bitwise_xor_expression = bitwise_and_expression (__ "^" __ bitwise_and_expression)* -> ();
    bitwise_and_expression = bitwise_shift_expression (__ "&" __ bitwise_shift_expression)* -> ();
    bitwise_shift_expression = additive_expression (__ ("<<" | ">>") __ additive_expression)* -> ();
    additive_expression = multiplicative_expression (__ ("+" | "-") __ multiplicative_expression)* -> ();
    multiplicative_expression = unary_expression (__ ("*" | "/" | "%") __ unary_expression)* -> ();
    unary_expression = ((("-" | "!") __ unary_expression) | primary_expression) -> ();
    primary_expression = ("(" __ expression __ ")" | literal | identifier) -> ();
    literal = #["0123456789"]+ -> ();
    identifier = #["ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_"]+ -> ();
}

fn main() {
    parser_repl(expression);
}
