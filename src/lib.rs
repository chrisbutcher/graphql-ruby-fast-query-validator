extern crate libc;

use std::ffi::CStr;
use std::str;
use libc::c_char;

use graphql_parser::parse_query;

#[no_mangle]
pub extern fn is_query_valid(value: *const c_char) -> bool {
    let c_value = unsafe { CStr::from_ptr(value).to_bytes() };

    match str::from_utf8(c_value) {
        Ok(value) => {
            match parse_query(value) {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn it_works_with_kitchen_sink_query() {
        let query = r#####"
        query queryName($foo: ComplexType, $site: Site = MOBILE) @onQuery {
            whoever123is: node(id: [123, 456]) {
                id ,
                ... on User @onInlineFragment {
                field2 {
                    id ,
                    alias: field1(first:10, after:$foo,) @include(if: $foo) {
                    id,
                    ...frag @onFragmentSpread
                    }
                }
                }
                ... @skip(unless: $foo) {
                id
                }
                ... {
                id
                }
            }
            }

            mutation likeStory @onMutation {
            like(story: 123) @onField {
                story {
                id @onField
                }
            }
            }

            subscription StoryLikeSubscription(
            $input: StoryLikeSubscribeInput
            ) @onSubscription {
            storyLikeSubscribe(input: $input) {
                story {
                likers {
                    count
                }
                likeSentence {
                    text
                }
                }
            }
            }

            fragment frag on Friend @onFragmentDefinition {
            foo(size: $size, bar: $b, obj: {key: "value", block: """

                block string uses \"""

            """})
            }

            {
            unnamed(truthy: true, falsey: false, nullish: null),
            query
            }

            query { __typename }
        "#####;

        assert_eq!(true, is_query_valid(CString::new(query).unwrap().as_ptr()));
    }
}
