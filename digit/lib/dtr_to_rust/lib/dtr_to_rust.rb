# frozen_string_literal: true

# This is the main module for the DTR to Rust gem.
module DTRToRust
  autoload :Generator, 'generator'
  autoload :InstructionHandler, 'instruction_handler'

  # This module contains all the classes that handle the different types of instructions.
  module Instruction
    autoload :AddAndAssign, 'instruction/add_and_assign'
    autoload :CreateList, 'instruction/create_list'
    autoload :Evaluate, 'instruction/evaluate'
    autoload :Field, 'instruction/field'
    autoload :Handler, 'instruction/handler'
    autoload :InitializeUDT, 'instruction/initialize_udt'
    autoload :LogString, 'instruction/log_string'
    autoload :Return, 'instruction/return'
  end

  # This module contains all the classes that handle common logic.
  module Common
    autoload :InputInterpreter, 'common/input_interpreter'
  end
end
