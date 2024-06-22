# frozen_string_literal: true

module DTRToRust
  # This class is responsible for generating Rust code for a single instruction.
  class InstructionHandler
    def initialize(instruction, spacing_scope, function_names, user_defined_types, is_helper)
      @instruction = instruction
      @spacing_scope = spacing_scope
      @function_names = function_names
      @user_defined_types = user_defined_types
      @is_helper = is_helper
    end

    def generate_rust
      unless EXPRESSION_FOOBAR.key?(@instruction.instruction.strip)
        raise "Unknown instruction type: #{@instruction.instruction}"
      end

      EXPRESSION_FOOBAR[@instruction.instruction.strip].send(:handle, @instruction, @spacing_scope, @function_names,
                                                             @user_defined_types, @is_helper)
    end

    private

    EXPRESSION_FOOBAR = {
      'assign' => Instruction::Assign,
      'jump' => Instruction::Jump,
      'goto' => Instruction::Goto,
      'exit_with_message' => Instruction::ExitWithMessage,
      'and' => Instruction::And,
      'or' => Instruction::Or,
      'label' => Instruction::Label,
      'add' => Instruction::Add,
      'subtract' => Instruction::Subtract,
      'multiply' => Instruction::Multiply,
      'divide' => Instruction::Divide,
      'instantiate_object' => Instruction::InstantiateObject,
      'print' => Instruction::Print,
      'return' => Instruction::Return,
      'evaluate' => Instruction::Evaluate,
      'field' => Instruction::Field
    }.freeze

    def handle_empty_instruction
      ''
    end
    attr_reader :instruction
  end
end
