# frozen_string_literal: true

module DTRCore
  # Instruction class
  class Instruction
    attr_reader :instruction, :inputs, :assign, :scope, :id

    def initialize(instruction, inputs, assign, scope, id)
      @instruction = instruction
      @inputs = inputs
      @assign = assign
      @scope = scope
      @id = id
    end

    def ==(other)
      instruction == other.instruction &&
        inputs == other.inputs &&
        assign == other.assign &&
        scope == other.scope &&
        id == other.id
    end

    def to_s
      assignment = @assign.nil? ? '' : "assign: #{@assign}, "
      "{ id: #{id}, instruction: #{instruction}, " \
        "input: (#{inputs&.join(', ')}), " \
        "#{assignment}scope: #{scope} }"
    end

    def to_json(*_args)
      {
        instruction:,
        inputs:,
        assign:,
        scope:,
        id:
      }.to_json
    end

    def valid?
      DTRCore::InstructionValidator.new(self).valid?
    end
  end
end
