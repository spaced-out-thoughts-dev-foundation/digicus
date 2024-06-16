# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Field instruction.
    class Field < Handler
      def handle
        form_rust_string("#{handle_field_assign}#{handle_field_call}", @instruction[:scope])
      end

      private

      def handle_field_call
        "#{@instruction[:inputs][0]}.#{@instruction[:inputs][1]};"
      end

      def handle_field_assign
        "let mut #{@instruction[:assign]} = " if @instruction[:assign]
      end
    end
  end
end
