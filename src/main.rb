require 'ffi'

module GraphQLQueryValidator
  extend FFI::Library
  # TODO: target/release
  ffi_lib 'target/debug/libgraphql_query_validator.' + FFI::Platform::LIBSUFFIX
  attach_function :is_query_valid, [ :string ], :bool
end

input = "{ shop { name } }"
output = GraphQLQueryValidator.is_query_valid(input)
puts "GraphQLQueryValidator.is_query_valid result: #{output}"
