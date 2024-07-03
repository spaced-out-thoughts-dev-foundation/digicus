# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the AddAndAssign instruction.
    class Handler
      def initialize(instruction, spacing_scope, function_names, user_defined_types, is_helper,
                     assignment_name_to_scope_map, function_inputs)
        @instruction = instruction
        @spacing_scope = spacing_scope
        @function_names = function_names
        @user_defined_types = user_defined_types
        @is_helper = is_helper
        @assignment_name_to_scope_map = assignment_name_to_scope_map
        @function_inputs = function_inputs

        format_assign
      end

      def format_assign
        unless @instruction.assign && @instruction.assign.include?('|||') && @instruction.assign.split('|||').size == 2
          return
        end

        var_name, type_name = @instruction.assign.split('|||')

        @instruction = DTRCore::Instruction.new(
          @instruction.instruction,
          @instruction.inputs,
          "#{var_name}:#{Common::TypeTranslator.translate_type(type_name)}",
          @instruction.scope
        )
      end

      def self.handle(instruction, spacing_scope, function_names, user_defined_types, is_helper, assignment_name_to_scope_map, function_inputs)
        new(instruction, spacing_scope, function_names, user_defined_types, is_helper,
            assignment_name_to_scope_map, function_inputs).handle
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
