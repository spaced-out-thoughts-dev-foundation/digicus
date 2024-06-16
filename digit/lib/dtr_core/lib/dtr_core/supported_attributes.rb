# frozen_string_literal: true

# rubocop:disable Layout/LineLength
module DTRCore
  module SupportedAttributes
    # Supported Instructions for DTR.
    ## Instruction Categories ##
    INSTRUCTION_CATEGORY_BASIC = 'basic'
    INSTRUCTION_CATEGORY_BINARY = 'binary'
    INSTRUCTION_CATEGORY_CONTROL_FLOW = 'control_flow'
    INSTRUCTION_CATEGORY_TERMINATING = 'terminating'
    INSTRUCTION_CATEGORY_LOGICAL = 'logical'
    INSTRUCTION_CATEGORY_OBJECT = 'object'
    INSTRUCTION_CATEGORIES = [
      INSTRUCTION_CATEGORY_BASIC,
      INSTRUCTION_CATEGORY_BINARY,
      INSTRUCTION_CATEGORY_CONTROL_FLOW,
      INSTRUCTION_CATEGORY_TERMINATING,
      INSTRUCTION_CATEGORY_LOGICAL,
      INSTRUCTION_CATEGORY_OBJECT
    ].freeze
    ## Instructions ##
    INSTRUCTIONS = [
      { name: 'assign', description: 'given some input value, assign to ASSIGN_NAME',
        category: INSTRUCTION_CATEGORY_BASIC },
      { name: 'evaluate',
        description: 'given a method name and 0 or more inputs, execute method. At this time, evaluate is a fairly loose catch-all for not explicitly defined operations', category: INSTRUCTION_CATEGORY_BASIC },
      { name: 'print', description: 'given some value, print it to standard out',
        category: INSTRUCTION_CATEGORY_BASIC },

      { name: 'exit_with_message', description: 'immediately end execution, returning message',
        category: INSTRUCTION_CATEGORY_TERMINATING },
      { name: 'return', description: 'return from function with input value',
        category: INSTRUCTION_CATEGORY_TERMINATING },

      { name: 'and', description: 'lassign to ASSIGN_NAME result of “and-ing” two values',
        category: INSTRUCTION_CATEGORY_LOGICAL },
      { name: 'or', description: 'assign to ASSIGN_NAME result of “or-ing” two values',
        category: INSTRUCTION_CATEGORY_LOGICAL },

      { name: 'goto',
        description: 'conditional if two inputs. In this case, first input is the condition to evaluate. If that is true, or there is only one input, move in code to the first input (a label name)', category: INSTRUCTION_CATEGORY_CONTROL_FLOW },
      { name: 'jump',
        description: 'conditional if two inputs. In this case, first input is the condition to evaluate. If that is true, or there is only one input, jump to scope level', category: INSTRUCTION_CATEGORY_CONTROL_FLOW },
      { name: 'end_of_iteration_check',
        description: 'check on input to see if at end of iteration. Return result to ASSIGN_NAME', category: INSTRUCTION_CATEGORY_CONTROL_FLOW },
      { name: 'label', description: 'a named location within the instruction set for a given function',
        category: INSTRUCTION_CATEGORY_CONTROL_FLOW },

      { name: 'field', description: 'access a field on an object and assign result to ASSIGN_NAME',
        category: INSTRUCTION_CATEGORY_OBJECT },
      { name: 'instantiate_object',
        description: 'initialize an object by first passing in the type of object and the passing in each initial values for its fields. Supported types here include: Dictionary, List, Range, Tuple, and UDT. For UDTs, the second input is the name of the UDT.', category: INSTRUCTION_CATEGORY_OBJECT },

      { name: 'add', description: 'assign to ASSIGN_NAME result of adding two value',
        category: INSTRUCTION_CATEGORY_BINARY },
      { name: 'subtract', description: 'assign to ASSIGN_NAME result of subtracting two value',
        category: INSTRUCTION_CATEGORY_BINARY },
      { name: 'multiply', description: 'assign to ASSIGN_NAME result of multiplying two value',
        category: INSTRUCTION_CATEGORY_BINARY },
      { name: 'divide', description: 'assign to ASSIGN_NAME result of dividing two value',
        category: INSTRUCTION_CATEGORY_BINARY }
    ].freeze

    # Supported Types for DTR.
    TYPES = %w[
      Dictionary
      List
      Range
      Tuple
      UDT
      Address
      BigInteger
      Boolean
      Float
      Integer
      String
    ].freeze
  end
end
# rubocop:enable Layout/LineLength
