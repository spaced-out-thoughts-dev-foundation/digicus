# frozen_string_literal: true

module DTRToRust
  module Optimization
    # This class is responsible for converting field instructions to assignment instructions
    class FieldToAssignmentConversion
      def initialize(instructions)
        @instructions = instructions
      end

      def self.apply(instructions)
        new(instructions).apply
      end

      def apply
        @instructions.map do |instruction|
          next instruction if skip_instruction?(instruction)

          apply_to_instruction(instruction)
        end
      end

      private

      def skip_instruction?(instruction)
        instruction.instruction != 'field'
      end

      def apply_to_instruction(instruction)
        return instruction unless instruction.inputs.length == 2

        DTRCore::Instruction.new('assign', ["#{instruction.inputs[0]}.#{instruction.inputs[1]}"], instruction.assign,
                                 instruction.scope)
      end
    end
  end
end
