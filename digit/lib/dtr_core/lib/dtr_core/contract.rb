# frozen_string_literal: true

module DTRCore
  class Contract
    attr_reader :functions, :name, :state

    def initialize(name, state, functions)
      @name = name
      @state = state
      @functions = functions
    end

    def ==(other)
      name == other.name &&
        state == other.state &&
        functions == other.functions
    end
  end
end
