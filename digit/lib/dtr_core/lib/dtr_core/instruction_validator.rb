# frozen_string_literal: true

require 'dtr_core/common'

module DTRCore
  # Instruction validator for DTR types.
  class InstructionValidator
    include ::DTRCore::Common

    def initialize(instruction)
      @instruction = instruction

      validate_input!
    end

    # rubocop:disable Metrics/CyclomaticComplexity
    # rubocop:disable Metrics/MethodLength
    def valid?
      return false unless scope_valid?

      case @instruction.instruction
      when 'assign', 'evaluate', 'print'
        validate_basic_operation!
      when 'exit_with_message', 'return'
        validate_terminating_operation!
      when 'and', 'or'
        validate_logical_operation!
      when 'goto', 'jump', 'end_of_iteration_check', 'label'
        validate_control_flow_operation!
      when 'field', 'instantiate_object'
        validate_object_operation!
      when 'add', 'subtract', 'multiply', 'divide'
        validate_binary_operation!
      when 'increment'
        true
      else
        false
      end
    end
    # rubocop:enable Metrics/MethodLength
    # rubocop:enable Metrics/CyclomaticComplexity

    private

    def validate_input!
      raise 'Missing Instruction.' if @instruction.nil?
      raise 'Instruction name missing.' if @instruction.instruction.nil?
      raise 'Instruction missing scope.' if @instruction.scope.nil?
    end

    def scope_valid?
      @instruction.scope >= 0
    end

    def validate_basic_operation!
      true
    end

    def validate_terminating_operation!
      true
    end

    def validate_logical_operation!
      @instruction.inputs&.length == 2
    end

    def validate_control_flow_operation!
      true
    end

    def validate_object_operation!
      true
    end

    def validate_binary_operation!
      @instruction.inputs&.length == 2
    end
  end
end
