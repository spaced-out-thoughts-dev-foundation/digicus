# frozen_string_literal: true

# This is the main module for the DTR to Rust gem.
module DTRToRust
  autoload :Generator, 'generator'
  autoload :InstructionHandler, 'instruction_handler'

  # This module contains all the classes that handle the different types of instructions.
  module Instruction
    autoload :Handler, 'instruction/handler'
    autoload :Evaluate, 'instruction/evaluate'
    autoload :Return, 'instruction/return'
    autoload :LogString, 'instruction/log_string'
    autoload :AddAndAssign, 'instruction/add_and_assign'
  end
end
