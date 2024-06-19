# frozen_string_literal: true

module DTRToRust
  # This class is responsible for generating Rust code for a single instruction.
  class InstructionHandler
    def initialize(instruction)
      @instruction = instruction
    end

    def generate_rust
      unless EXPRESSION_FOOBAR.key?(@instruction.instruction.strip)
        raise "Unknown instruction type: #{@instruction.instruction}"
      end

      EXPRESSION_FOOBAR[@instruction.instruction.strip].call(@instruction)
    end

    private

    EXPRESSION_FOOBAR = {
      'assign' => Instruction::Assign.handle,
      'jump' => Instruction::Jump.handle,
      'goto' => Instruction::Goto.handle,
      'error_with_message' => Instruction::ErrorWithMessage.handle,
      'and' => Instruction::And.handle,
      'or' => Instruction::Or.handle,
      'label' => Instruction::Label.handle,
      'add' => Instruction::Add.handle,
      'subtract' => Instruction::Subtract.handle,
      'multiply' => Instruction::Multiply.handle,
      'divide' => Instruction::Divide.handle,
      'instantiate_object' => Instruction::InstantiateObject.handle,
      'print' => Instruction::Print.handle,
      'return' => Instruction::Return.handle,
      'evaluate' => Instruction::Evaluate.handle,
      'field' => Instruction::Field.handle
    }.freeze

    def handle_empty_instruction
      ''
    end
    attr_reader :instruction
  end
end
