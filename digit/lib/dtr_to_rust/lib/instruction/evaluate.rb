# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Evaluate instruction.
    class Evaluate < Handler
      def handle
        rust_string = "#{@instruction[:assign]} = #{@instruction[:inputs][0]}(#{@instruction[:inputs][1..].join(', ')})"

        form_rust_string(rust_string, @instruction[:scope])
      end
    end
  end
end
