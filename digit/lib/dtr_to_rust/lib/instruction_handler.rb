# frozen_string_literal: true

module DTRToRust
  # This class is responsible for generating Rust code for a single instruction.
  class InstructionHandler
    def initialize(instruction)
      @instruction = instruction
    end

    def generate_rust
      case @instruction[:instruction]
      when 'AddSymbols'
        handle_add_symbols
      when 'Return'
        handle_return
      else
        raise "Unknown instruction type: #{@instruction[:instruction]}"
      end
    end

    private

    attr_reader :instruction

    def handle_add_symbols
      "      let #{instruction[:assign]} " \
        '= vec![&env, ' \
        "symbol_short!(#{instruction[:inputs][0]}), #{instruction[:inputs][1]}];"
    end

    def handle_return
      "      #{instruction[:inputs][0]}"
    end
  end
end
