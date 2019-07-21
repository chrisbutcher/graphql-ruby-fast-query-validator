# graphql-ruby-fast-query-validator

Experimental Ruby + Rust FFI library to quickly determine if a GraphQL query is (syntactically) valid.

```ruby
input = "{ shop { name } }"
output = GraphQLQueryValidator.is_query_valid(input)
# => true

input = "foobar"
output = GraphQLQueryValidator.is_query_valid(input)
# => false
```

# Credits / dependencies

* https://github.com/graphql-rust/graphql-parser
* https://github.com/Marwes/combine
* https://github.com/ffi/ffi
