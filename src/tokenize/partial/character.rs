use internal::*;
use debug::*;

use tokenize::Token;

pub struct CharacterTokenizer {
    delimiters:     Vec<(SharedString, SharedString)>,
    replace:        Vec<(SharedString, SharedString)>,
}

impl CharacterTokenizer {

    pub fn new(settings: &Data, character_stack: &mut CharacterStack, variant_registry: &mut VariantRegistry) -> Status<Self> {
        ensure!(settings.is_map(), ExpectedFound, expected_list!["map"], settings.clone());
        variant_registry.has_characters = true;
        let mut delimiters = Vec::new();
        let mut replace = Vec::new();

        if let Some(block_comment) = confirm!(settings.index(&keyword!("delimiters"))) {
            for delimiter_list in unpack_list!(&block_comment).iter() {
                let delimiter_list = unpack_list!(delimiter_list);
                ensure!(delimiter_list.len() == 2, InvalidItemCount, integer!(2), integer!(delimiter_list.len() as i64));
                let start_delimiter = unpack_literal!(&delimiter_list[0]);
                let end_delimiter = unpack_literal!(&delimiter_list[1]);
                ensure!(!start_delimiter.is_empty(), EmptyLiteral);
                ensure!(!end_delimiter.is_empty(), EmptyLiteral);
                confirm!(character_stack.register_breaking(start_delimiter[0]));
                confirm!(character_stack.register_signature(start_delimiter.clone()));
                push_by_length!(delimiters, start_delimiter, end_delimiter);
            }
        }

        if let Some(replace_lookup) = confirm!(settings.index(&keyword!("replace"))) {
            ensure!(replace_lookup.is_map(), ExpectedFound, expected_list!["map"], replace_lookup.clone());

            for (from, to) in confirm!(replace_lookup.pairs()).into_iter() {
                let from = unpack_literal!(&from);
                let to = unpack_literal!(&to);
                ensure!(!from.is_empty(), EmptyLiteral);
                push_by_length!(replace, from, to);
            }
        }

        return success!(Self {
            delimiters:     delimiters,
            replace:        replace,
        });
    }

    pub fn find(&self, character_stack: &mut CharacterStack, tokens: &mut Vec<Token>) -> Status<bool> {
        for (start_delimiter, end_delimiter) in self.delimiters.iter() {
            if character_stack.check_string(&start_delimiter) {
                let mut character = SharedString::new();

                'check: while !character_stack.check_string(&end_delimiter) {

                    if character_stack.is_empty() {
                        let error = Error::UnterminatedToken(identifier!("character"));
                        tokens.push(Token::new(TokenType::Invalid(error), character_stack.final_positions()));
                        return success!(true);
                    }

                    for (from, to) in self.replace.iter() {
                        if character_stack.check_string(&from) {
                            character.push_str(to);
                            continue 'check;
                        }
                    }

                    character.push(character_stack.pop().unwrap());
                }

                if character.len() != 1 {
                    let error = Error::InvalidCharacterLength(string!(String, character));
                    tokens.push(Token::new(TokenType::Invalid(error), character_stack.final_positions()));
                    return success!(true);
                }

                tokens.push(Token::new(TokenType::Character(character[0]), character_stack.final_positions()));
                return success!(true);
            }
        }
        
        return success!(false);
    }
}
