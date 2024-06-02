# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the LogString instruction.
    class LogString < Handler
      def handle
        form_rust_string("log!(#{@instruction[:inputs].join(',')})", @instruction[:scope])
      end
    end
  end
end
