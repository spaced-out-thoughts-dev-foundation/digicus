# frozen_string_literal: true

# This is the main module for the DTR to Rust gem.
module DTRToRust
  autoload :Generator, 'generator'
  autoload :InstructionHandler, 'instruction_handler'

  # This module contains all the classes that handle the different types of instructions.
  module Instruction
    autoload :Evaluate, 'instruction/evaluate'
    autoload :Field, 'instruction/field'
    autoload :Handler, 'instruction/handler'
    autoload :Print, 'instruction/print'
    autoload :Return, 'instruction/return'
    autoload :InstantiateObject, 'instruction/instantiate_object'
    autoload :Add, 'instruction/add'
    autoload :Subtract, 'instruction/subtract'
    autoload :Multiply, 'instruction/multiply'
    autoload :Divide, 'instruction/divide'
    autoload :Assign, 'instruction/assign'
    autoload :Jump, 'instruction/jump'
    autoload :Goto, 'instruction/goto'
    autoload :ExitWithMessage, 'instruction/exit_with_message'
    autoload :And, 'instruction/and'
    autoload :Or, 'instruction/or'
    autoload :Label, 'instruction/label'
    autoload :EndOfIterationCheck, 'instruction/end_of_iteration_check'
    autoload :Increment, 'instruction/increment'
  end

  # This module contains all the classes that handle common logic.
  module Common
    autoload :InputInterpreter, 'common/input_interpreter'
    autoload :ReferenceAppender, 'common/reference_appender'
    autoload :TypeTranslator, 'common/type_translator'
  end

  # This module contains all the classes that handle optimization.
  module Optimization
    autoload :ChainedInvocationAssignmentReduction, 'optimization/chained_invocation_assignment_reduction'
    autoload :FieldToAssignmentConversion, 'optimization/field_to_assignment_conversion'
    autoload :BinaryXToSelfAssignmentReduction, 'optimization/binary_x_to_self_assignment_reduction'
  end

  # This module contains all the classes that handle user defined types.
  module UserDefinedTypes
    autoload :Handler, 'user_defined_types/handler'
  end

  # This module contains all the classes that handle the aggregation of instructions.
  module Aggregator
    autoload :ScopeBlockAggregator, 'aggregator/scope_block_aggregator'
    autoload :LoopAggregator, 'aggregator/loop_aggregator'
  end
end
