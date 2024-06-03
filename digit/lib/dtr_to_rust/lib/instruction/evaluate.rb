# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Evaluate instruction.
    class Evaluate < Handler
      def handle
        rust_string = if @instruction[:assign].nil?
          "#{@instruction[:inputs][0]}(#{@instruction[:inputs][1..].join(', ')});"
        else
          "let mut #{@instruction[:assign]} = #{@instruction[:inputs][0]}(#{@instruction[:inputs][1..].join(', ')});"
        end

        form_rust_string(rust_string, @instruction[:scope])
      end
    end
  end
end
