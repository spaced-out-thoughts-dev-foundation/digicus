# frozen_string_literal: true

# Core logic for consuming Digicus Textual Representation (DTR) files.
module DTRCore
  autoload :Contract, 'dtr_core/contract'
  autoload :Function, 'dtr_core/function'
  autoload :Number, 'dtr_core/number'
  autoload :Parser, 'dtr_core/parser'
  autoload :State, 'dtr_core/state'
  autoload :SupportedAttributes, 'dtr_core/supported_attributes'
  autoload :TypeValidator, 'dtr_core/type_validator'
  autoload :InstructionValidator, 'dtr_core/instruction_validator'
  autoload :Instruction, 'dtr_core/instruction'
  autoload :UserDefinedType, 'dtr_core/user_defined_type'
end
