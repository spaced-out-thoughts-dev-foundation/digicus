# frozen_string_literal: true

module Digicus
  class Compiler
    def initialize
      @parser = Digicus::Parser.new
    end

    def compile(code)
      @parser.parse(code)
    end
  end
end
