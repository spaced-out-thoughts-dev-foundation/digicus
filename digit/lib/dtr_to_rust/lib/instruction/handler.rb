# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the AddAndAssign instruction.
    class Handler
      def initialize(instruction, spacing_scope, function_names, user_defined_types, is_helper)
        @instruction = instruction
        @spacing_scope = spacing_scope
        @function_names = function_names
        @user_defined_types = user_defined_types
        @is_helper = is_helper
      end

      def self.handle(instruction, spacing_scope, function_names, user_defined_types, is_helper)
        new(instruction, spacing_scope, function_names, user_defined_types, is_helper).handle
      end

      def spacing
        '    ' * (@is_helper ? 1 : @spacing_scope + 2)
      end

      def form_rust_string(instruction_string)
        "#{spacing}#{instruction_string}"
      end
    end
  end
end
