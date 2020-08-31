use crate::lexer::Lexer;
use crate::resource::Resource;
use crate::token_type::TokenType;
use std::collections::HashMap;

pub(crate) struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Option<Box<TokenType>>,
    peek_token: Option<Box<TokenType>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(Box::new(self.lexer.next()));
    }
    pub(crate) fn parse_resources(&mut self) -> Vec<Resource> {
        let mut resources: Vec<Resource> = vec![];
        while self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::EOF)
        {
            resources.push(self.parse_resource())
        }
        resources
    }
    fn parse_resource(&mut self) -> Resource {
        let resource_reserved_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let resource_reserved_literal = match resource_reserved_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        if resource_reserved_literal != "resource" {
            panic!("token is invalid. expect resource");
        }
        self.next_token();

        let resource_name_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let resource_name_literal = match resource_name_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        if resource_name_literal != "aws_sns_topic_subscription" {
            panic!("token is invalid. expect aws_sns_topic_subscription");
        }
        self.next_token();

        let event_name_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let event_name_literal = match event_name_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        self.next_token();

        let lbrace_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if &TokenType::LBrace != lbrace_token.as_ref() {
            panic!("token is invalid. expect lbrace");
        }
        self.next_token();

        let attributes = self.parse_attributes();

        let rbrace_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if &TokenType::RBrace != rbrace_token.as_ref() {
            panic!("token is invalid. expect rbrace");
        }
        self.next_token();

        Resource::new(event_name_literal.to_owned(), attributes)
    }
    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::<String, String>::new();
        while self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::RBrace)
        {
            let (key, value) = self.parse_attribute();
            attributes.insert(key, value);
        }
        attributes
    }
    fn parse_attribute(&mut self) -> (String, String) {
        let key_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let key_literal = match key_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        self.next_token();

        if self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::Equal)
        {
            panic!("token is invalid. expect TokenType::Equal");
        }
        self.next_token();

        let value_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let value_literal = match value_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        self.next_token();

        (key_literal.clone(), value_literal.clone())
    }
}