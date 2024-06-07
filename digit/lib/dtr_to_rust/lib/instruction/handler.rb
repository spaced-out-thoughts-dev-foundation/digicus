# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the AddAndAssign instruction.
    class Handler
      def initialize(instruction)
        @instruction = instruction
      end

      def self.handle(instruction)
        new(instruction).handle
      end

      def spacing(scope)
        '    ' * (scope + 1)
      end

      def form_rust_string(instruction_string, scope)
        "#{spacing(scope)}#{instruction_string}"
      end
    end
  end
end
