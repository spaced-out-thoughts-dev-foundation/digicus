# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the AddAndAssign instruction.
    class CreateList < Handler
      def handle
        form_rust_string("let #{@instruction[:assign]} = vec![#{handle_inputs}];",
                         @instruction[:scope])
      end

      private

      def handle_inputs
        @instruction[:inputs].map { |input| handle_input(input) }.join(', ')
      end

      def handle_input(input)
        if (input.start_with?('"') && input[-1] == ('"'))
          "symbol_short!(#{input})"
        else
          input
        end
      end
    end
  end
end
