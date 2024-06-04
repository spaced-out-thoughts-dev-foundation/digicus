# frozen_string_literal: true

module DTRCore
  module SupportedAttributes
    # Supported Instructions for DTR.
    ## Instruction Categories ##
    INSTRUCTION_CATEGORY_BASIC = 'basic'
    INSTRUCTION_CATEGORY_STATE = 'state'
    INSTRUCTION_CATEGORY_UNTYPED = 'untyped'
    INSTRUCTION_CATEGORY_NUMERIC = 'numeric'
    INSTRUCTION_CATEGORY_STRING = 'string'
    INSTRUCTION_CATEGORY_ENVIRONMENT = 'environment'
    INSTRUCTION_CATEGORY_METHODS = 'methods'
    INSTRUCTION_CATEGORY_OBJECTS = 'objects'
    INSTRUCTION_CATEGORY_CONDITIONAL = 'conditional'
    INSTRUCTION_CATEGORY_LOGICAL = 'logical'
    INSTRUCTION_CATEGORIES = [
      INSTRUCTION_CATEGORY_BASIC,
      INSTRUCTION_CATEGORY_STATE,
      INSTRUCTION_CATEGORY_UNTYPED,
      INSTRUCTION_CATEGORY_NUMERIC,
      INSTRUCTION_CATEGORY_STRING,
      INSTRUCTION_CATEGORY_ENVIRONMENT,
      INSTRUCTION_CATEGORY_METHODS,
      INSTRUCTION_CATEGORY_OBJECTS,
      INSTRUCTION_CATEGORY_CONDITIONAL,
      INSTRUCTION_CATEGORY_LOGICAL
    ].freeze
    ## Instructions ##
    INSTRUCTIONS = [
      # basic operations
      { name: 'return', description: 'Return a value from a function.', category: INSTRUCTION_CATEGORY_BASIC },
      { name: 'assign', description: 'Assign a value to a variable.', category: INSTRUCTION_CATEGORY_BASIC },
      { name: 'panic', description: 'Exit, quickly, and loudly.', category: INSTRUCTION_CATEGORY_BASIC },
      # state operations
      { name: 'fetch_state', description: 'Fetch a value from the state.', category: INSTRUCTION_CATEGORY_STATE },
      { name: 'save_state', description: 'Save a value to the state.', category: INSTRUCTION_CATEGORY_STATE },
      # untyped operations
      { name: 'add', description: 'Add two things of unknown types together.', category: INSTRUCTION_CATEGORY_UNTYPED },
      { name: 'add_and_assign',
        description: 'Add two things of unknown types together and then assign to the first one.',
        category: INSTRUCTION_CATEGORY_UNTYPED },
      { name: 'subtract', description: 'Subtract two things of unknown types together.', category: INSTRUCTION_CATEGORY_UNTYPED },
      { name: 'divide', description: 'Divide two things of unknown types together.', category: INSTRUCTION_CATEGORY_UNTYPED },
      { name: 'multiply', description: 'Multiply two things of unknown types together.', category: INSTRUCTION_CATEGORY_UNTYPED },
      # numeric operations
      { name: 'add_numbers', description: 'Add two numbers.', category: INSTRUCTION_CATEGORY_NUMERIC },
      { name: 'subtract_numbers', description: 'Subtract two numbers.', category: INSTRUCTION_CATEGORY_NUMERIC },
      { name: 'multiply_numbers', description: 'Multiply two numbers.', category: INSTRUCTION_CATEGORY_NUMERIC },
      { name: 'divide_numbers', description: 'Divide two numbers.', category: INSTRUCTION_CATEGORY_NUMERIC },
      # string operations
      { name: 'add_strings', description: 'Concatenate two strings.', category: INSTRUCTION_CATEGORY_STRING },
      # environment operations
      { name: 'contract_address', description: 'Get the contract address.',
        category: INSTRUCTION_CATEGORY_ENVIRONMENT },
      # method operations
      { name: 'evaluate', description: 'Evaluate a method. Method name is the first input and arguments follow',
        category: INSTRUCTION_CATEGORY_METHODS },
      # object operations
      { name: 'field', description: 'Reference an object field.', category: INSTRUCTION_CATEGORY_OBJECTS },
      { name: 'initialize_udt', description: 'Instantiate UDT object.', category: INSTRUCTION_CATEGORY_OBJECTS },
      # conditional operations
      { name: 'conditional_jump', description: 'Jump to a label if first input is true.', category: INSTRUCTION_CATEGORY_CONDITIONAL },
      { name: 'unconditional_jump', description: 'Jump to a no matter what.', category: INSTRUCTION_CATEGORY_CONDITIONAL },
      # logical operations
      { name: 'and', description: 'Logical AND.', category: INSTRUCTION_CATEGORY_LOGICAL },
      { name: 'or', description: 'Logical OR.', category: INSTRUCTION_CATEGORY_LOGICAL }
    ].freeze

    # Supported Types for DTR.
    TYPES = [
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
      'i128',
      'i256',
      ## unsigned
      'u32',
      'u64',
      'u128',
      'u256'
    ].freeze
  end
end
