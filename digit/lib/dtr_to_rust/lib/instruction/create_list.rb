# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the AddAndAssign instruction.
    class CreateList < Handler
      def handle
        form_rust_string("let #{@instruction[:assign]} = vec![#{@instruction[:inputs].join(', ')}];",
                         @instruction[:scope])
      end
    end
  end
end
