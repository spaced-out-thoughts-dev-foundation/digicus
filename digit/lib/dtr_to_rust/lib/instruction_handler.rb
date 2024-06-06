# frozen_string_literal: true

module DTRToRust
  # This class is responsible for generating Rust code for a single instruction.
  class InstructionHandler
    def initialize(instruction)
      @instruction = instruction
    end

    def generate_rust
      case @instruction[:instruction]
      when 'Return'
        Instruction::Return.handle(@instruction)
      when 'log_string'
        Instruction::LogString.handle(@instruction)
      when 'add_and_assign'
        Instruction::AddAndAssign.handle(@instruction)
      when 'evaluate'
        Instruction::Evaluate.handle(@instruction)
      when 'create_list'
        Instruction::CreateList.handle(@instruction)
      else
        raise "Unknown instruction type: #{@instruction[:instruction]}"
      end
    end

    private

    attr_reader :instruction
  end
end
