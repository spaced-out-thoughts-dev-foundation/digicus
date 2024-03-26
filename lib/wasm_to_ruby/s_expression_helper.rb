# frozen_string_literal: true

module WasmToRuby
  module SExpressionHelper
    def self.make_levels(input)
      make_levels_internal(input.split('')).first.first
    end

    def self.make_levels_internal(input, position = 0)
      stack = []
      values = input
      cur_text = ''

      while position <= values.length
        char = values[position]

        case char
        when '('
          stack_inside, position_new = make_levels_internal(values, position + 1)
          stack.push(stack_inside)
          position = position_new
        when ')'
          stack.push(cur_text.strip)
          return [stack.reverse, position + 1]
        else
          cur_text += char if char.nil? == false
          position += 1
        end
      end

      [stack.reverse, position]
    end
  end
end
