use crate::error;
use crate::expr::{Expr, LiteralValue};
use crate::statement::Stmt;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type ParseResult<T> = Result<T, String>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        let mut has_parse_error = false;
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    has_parse_error = true;
                    eprintln!("{}", e)
                }
            }
        }

        if has_parse_error {
            Err("Parsing error(s) found".to_string())
        } else {
            Ok(stmts)
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> Option<&Token> {
        let matched = self
            .tokens
            .get(self.current)
            .filter(|token| types.contains(&token.tag));

        if matched.is_some() {
            self.current += 1;
        }
        matched
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) if token.tag.eq(&TokenType::EOF) => true,
            None => true,
            _ => false,
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.is_at_end() {
            None
        } else {
            self.current += 1;
            self.tokens.get(self.current - 1)
        }
    }

    fn consume(&mut self, token: &TokenType, err_message: &str) -> ParseResult<()> {
        let res = self
            .peek()
            .filter(|t| t.tag.eq(token))
            .ok_or_else(|| err_message.to_owned())
            .map(|_| ());

        if res.is_ok() {
            self.current += 1;
        }

        res
    }

    fn consume_identifier(&mut self, err_message: &str) -> ParseResult<String> {
        let res = self
            .peek()
            .and_then(|t| t.tag.get_identifier_value())
            .ok_or_else(|| err_message.to_owned());

        if res.is_ok() {
            self.current += 1;
        }

        res
    }

    fn declaration(&mut self) -> ParseResult<Stmt> {
        let stmt = if self.match_token(&[TokenType::Var]).is_some() {
            self.finish_var_declaration()
        } else {
            self.statement()
        };

        if stmt.is_err() {
            self.synchronize();
        }

        stmt
    }

    fn finish_var_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume_identifier("expected an identifer after 'var' keyword")?;
        let initializer = if self.match_token(&[TokenType::Equal]).is_some() {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            &TokenType::Semicolon,
            "expected a semicolon following statement",
        )?;
        Ok(Stmt::VarDec { name, initializer })
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        if self.match_token(&[TokenType::Print]).is_some() {
            self.finish_print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn finish_print_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(
            &TokenType::Semicolon,
            "expected a semicolon following statement",
        )?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(
            &TokenType::Semicolon,
            "expected a semicolon following statement",
        )?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.ternary()?;

        if self.match_token(&[TokenType::Equal]).is_some() {
            let value = self.assignment()?;

            if let Expr::Variable(token) = expr {
                return Ok(Expr::Assign {
                    name: token,
                    value: Box::new(value),
                });
            } else {
                eprintln!("invalid assignment target");
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let operators = &[TokenType::BangEqual, TokenType::EqualEqual];
        if let Some(operator) = self.match_token(operators) {
            error::report(
                operator.line,
                "missing left-hand-side operand for equality expression",
            );
            self.comparison().and_then(|_| self.expression())
        } else {
            let mut expr = self.comparison()?;

            while let Some(operator) = self.match_token(operators) {
                let op = operator.clone();
                let right_expr = self.comparison()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator: op,
                    right: Box::new(right_expr),
                }
            }

            Ok(expr)
        }
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let operators = &[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        if let Some(operator) = self.match_token(operators) {
            error::report(
                operator.line,
                "missing left-hand-side operand for comparison expression",
            );
            self.comparison().and_then(|_| self.expression())
        } else {
            let mut expr = self.addition()?;

            while let Some(operator) = self.match_token(operators) {
                let op = operator.clone();
                let right_expr = self.addition()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator: op,
                    right: Box::new(right_expr),
                }
            }

            Ok(expr)
        }
    }

    fn addition(&mut self) -> ParseResult<Expr> {
        let operators = &[TokenType::Plus, TokenType::Minus];
        if let Some(operator) = self.match_token(&[TokenType::Plus]) {
            error::report(
                operator.line,
                "missing left-hand-side operand for addition expression",
            );
            self.comparison().and_then(|_| self.expression())
        } else {
            let mut expr = self.multiplication()?;

            while let Some(operator) = self.match_token(operators) {
                let op = operator.clone();
                let right_expr = self.multiplication()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator: op,
                    right: Box::new(right_expr),
                }
            }

            Ok(expr)
        }
    }

    fn multiplication(&mut self) -> ParseResult<Expr> {
        let operators = &[TokenType::Slash, TokenType::Star];
        if let Some(operator) = self.match_token(operators) {
            error::report(
                operator.line,
                "missing left-hand-side operand for multiplication expression",
            );
            self.comparison().and_then(|_| self.expression())
        } else {
            let mut expr = self.unary()?;

            while let Some(operator) = self.match_token(operators) {
                let op = operator.clone();
                let right_expr = self.unary()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator: op,
                    right: Box::new(right_expr),
                }
            }

            Ok(expr)
        }
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        match self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            Some(token) => Ok(Expr::Unary {
                operator: (token.clone()),
                operand: Box::new(self.unary()?),
            }),
            None => self.primary(),
        }
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        let token = self.peek();
        let pair = token.map(|t| (&t.tag, t.line));
        match pair {
            Some((TokenType::True, _)) => {
                self.current += 1;
                Ok(Expr::Literal(LiteralValue::True))
            }
            Some((TokenType::False, _)) => {
                self.current += 1;
                Ok(Expr::Literal(LiteralValue::False))
            }
            Some((TokenType::Number(n), _)) => {
                let num = *n;
                self.current += 1;
                Ok(Expr::Literal(LiteralValue::Number(num)))
            }
            Some((TokenType::Nil, _)) => {
                self.current += 1;
                Ok(Expr::Literal(LiteralValue::Nil))
            }
            Some((TokenType::STRING(val), _)) => {
                let s = val.to_owned();
                self.current += 1;
                Ok(Expr::Literal(LiteralValue::STRING(s)))
            }
            Some((TokenType::Identifer(_), _)) => {
                let token_c = token.unwrap().clone();
                self.current += 1;
                Ok(Expr::Variable(token_c))
            }
            Some((TokenType::LeftParen, line)) => {
                self.current += 1;
                let expr = self.expression()?;
                match self.consume(&TokenType::RightParen, "expected ')' after expression") {
                    Ok(_) => Ok(Expr::Grouping {
                        expr: Box::new(expr),
                    }),
                    Err(e) => {
                        error::report(line, &e);
                        Err(e)
                    }
                }
            }
            Some((token_type, line)) => {
                let message = format!("unexpected '{}'", token_type);
                error::report(line, &message);
                Err(message)
            }
            None => {
                let message = "unexpected end of input".to_owned();
                error::report(0, &message);
                Err(message)
            }
        }
    }

    fn ternary(&mut self) -> ParseResult<Expr> {
        let condition = self.equality()?;
        match self.match_token(&[TokenType::QuestionMark]) {
            Some(token) => {
                let line = token.line;
                let true_expr = self.equality()?;
                let consumed = self.consume(
                    &TokenType::Colon,
                    &format!("uh oh expected ':' in ternary expression, line: {}", line),
                );
                match consumed {
                    Ok(_) => {
                        let false_expr = self.equality()?;
                        let expr = Expr::Ternary {
                            condition: Box::new(condition),
                            true_expr: Box::new(true_expr),
                            false_expr: Box::new(false_expr),
                        };
                        Ok(expr)
                    }
                    Err(e) => {
                        error::report(line, &e);
                        Err(e)
                    }
                }
            }
            _ => Ok(condition),
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.peek() {
            match token.tag {
                TokenType::EOF | TokenType::Semicolon => {
                    self.advance();
                    return;
                }
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Expr, LiteralValue};

    #[test]
    fn test_unary_minus() {
        let tokens = vec![
            Token::new(TokenType::Minus, 0),
            Token::new(TokenType::Number(9.0), 0),
            Token::new(TokenType::Semicolon, 0),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result,
            Ok(vec![Stmt::Expr(Expr::Unary {
                operator: Token {
                    tag: TokenType::Minus,
                    line: 0
                },
                operand: Box::new(Expr::Literal(LiteralValue::Number(9.0))),
            })],)
        )
    }

    #[test]
    fn test_unary_bang() {
        let tokens = vec![
            Token::new(TokenType::Bang, 0),
            Token::new(TokenType::Number(10.0), 0),
            Token::new(TokenType::Semicolon, 0),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result,
            Ok(vec![Stmt::Expr(Expr::Unary {
                operator: Token {
                    tag: TokenType::Bang,
                    line: 0
                },
                operand: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            })],)
        )
    }

    #[test]
    fn test_chained_binary_operations() {
        let tokens = vec![
            Token::new(TokenType::Number(10.0), 0),
            Token::new(TokenType::Plus, 0),
            Token::new(TokenType::Number(2.0), 0),
            Token::new(TokenType::Star, 0),
            Token::new(TokenType::Number(6.0), 0),
            Token::new(TokenType::Semicolon, 0),
        ];

        let tokens_2 = vec![
            Token::new(TokenType::Number(4.0), 0),
            Token::new(TokenType::Star, 0),
            Token::new(TokenType::Number(24.0), 0),
            Token::new(TokenType::Plus, 0),
            Token::new(TokenType::Number(11.0), 0),
            Token::new(TokenType::Semicolon, 0),
        ];
        let mut parser = Parser::new(tokens);
        let mut parser_2 = Parser::new(tokens_2);
        let result = parser.parse();
        let result_2 = parser_2.parse();

        assert!(result.is_ok());
        assert!(result_2.is_ok());
        assert_eq!(
            result,
            Ok(vec![Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
                operator: Token {
                    tag: TokenType::Plus,
                    line: 0
                },
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
                    operator: Token {
                        tag: TokenType::Star,
                        line: 0
                    },
                    right: Box::new(Expr::Literal(LiteralValue::Number(6.0))),
                }),
            })],)
        );

        assert_eq!(
            result_2,
            Ok(vec![Stmt::Expr(Expr::Binary {
                right: Box::new(Expr::Literal(LiteralValue::Number(11.0))),
                operator: Token {
                    tag: TokenType::Plus,
                    line: 0
                },
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(LiteralValue::Number(4.0))),
                    operator: Token {
                        tag: TokenType::Star,
                        line: 0
                    },
                    right: Box::new(Expr::Literal(LiteralValue::Number(24.0))),
                }),
            })],)
        );
    }
}
