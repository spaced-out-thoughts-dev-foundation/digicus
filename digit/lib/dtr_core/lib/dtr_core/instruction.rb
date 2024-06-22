# frozen_string_literal: true

module DTRCore
  # Instruction class
  class Instruction
    attr_reader :instruction, :inputs, :assign, :scope

    def initialize(instruction, inputs, assign, scope)
      @instruction = instruction
      @inputs = inputs
      @assign = assign
      @scope = scope
    end

    def ==(other)
      instruction == other.instruction &&
        inputs == other.inputs &&
        assign == other.assign &&
        scope == other.scope
    end

    def to_s
      "{ instruction: #{instruction}, " \
        "input: (#{inputs&.join(', ')}), " \
        "assign: #{assign}, scope: #{scope} }"
    end

    def to_json(*_args)
      {
        instruction:,
        inputs:,
        assign:,
        scope:
      }.to_json
    end

    def valid?
      DTRCore::InstructionValidator.new(self).valid?
    end
  end
end
