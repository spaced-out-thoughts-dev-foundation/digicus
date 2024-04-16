# frozen_string_literal: true

module DTRCore
  # Represents a state in a DTR file.
  class Function
    attr_reader :name, :inputs, :output, :instructions

    def initialize(name, inputs, output, instructions)
      @name = name
      @inputs = inputs
      @output = output
      @instructions = instructions
    end

    def ==(other)
      name == other.name &&
        inputs == other.inputs &&
        output == other.output &&
        instructions == other.instructions
    end
  end
end
