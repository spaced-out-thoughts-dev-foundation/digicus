# frozen_string_literal: true

module Digit
  # Represents a contract in the Digit language
  class Contract
    attr_reader :name, :state, :functions

    def initialize(name, state, functions)
      @name = name
      @state = state
      @functions = functions
    end
  end
end
