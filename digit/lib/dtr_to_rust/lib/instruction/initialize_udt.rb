# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Field instruction.
    class InitializeUDT < Handler
      def handle
        form_rust_string("#{handle_assign}#{handle_udt_part}", @instruction[:scope])
      end

      private

      def handle_udt_part
        "#{@instruction[:inputs][0]} { #{@instruction[:inputs][1..].join(' ')} };"
      end

      def handle_assign
        "let mut #{@instruction[:assign]} = " if @instruction[:assign]
      end
    end
  end
end
