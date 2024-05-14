Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
    supported_instructions: [
      # basic operations
      'return',
      # state operations
      'fetch_state',
      'save_state',
      # numeric operations
      'add',
      'subtract',
      'multiply',
      'divide',
      # string operations
      'add_symbol',
    ],
    supported_types: [
      # basic types
      'address',
      'boolean',
      # string types
      'symbol',
      # collection types
      'array',
      'map',
      # numeric types
      ## signed
      'i32',
      'i64',
      'i128'
      'i256',
      ## unsigned
      'u32',
      'u64',
      'u128',
      'u256',
    ]
  }.to_json
end
