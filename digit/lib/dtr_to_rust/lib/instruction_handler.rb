module DTRToRust
  class InstructionHandler
    def initialize(instruction)
      @instruction = instruction
    end

    def generate_rust
      case @instruction[:instruction]
      when 'AddSymbols'
        "        add();"
      when 'Return'
        "        return();"
      else
        raise "Unknown instruction type: #{@instruction[:instruction]}"
      end
    end
  end
end