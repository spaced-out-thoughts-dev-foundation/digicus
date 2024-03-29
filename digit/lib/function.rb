# frozen_string_literal: true

module Digit
  # Represents a function in the Digit language
  class Function
    def initialize(name, inputs, outputs)
      @name = name
      @inputs = inputs
      @outputs = outputs
    end
  end
end
