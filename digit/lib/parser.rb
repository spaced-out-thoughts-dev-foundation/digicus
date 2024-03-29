# frozen_string_literal: true

module Digit
  # Rust parser
  class Parser
    def initialize(filepath)
      @filepath = filepath
      @content = File.read(@filepath)
      @cur_pos = 0
      @tokens = []
    end

    def self.parse(filepath)
      new(filepath).parse
    end

    def parse
      while @cur_pos < @content.length
        case @content[@cur_pos]
        when '#'
          @cur_pos += 1
          @tokens << { type: parse_comment }
        else
          parse_misc
        end
      end

      @tokens
    end

    private

    def parse_comment
      comment_value = ''
      while @content[@cur_pos] != "\n"
        comment_value += @content[@cur_pos]
        @cur_pos += 1
      end

      interpret_comment(comment_value)
    end

    def interpret_comment(comment_value)
      case comment_value
      when '![no_std]'
        :no_std_header
      when '[contract]'
        :contract_header
      when '[contractimpl]'
        :contract_impl_header
      else
        raise "Invalid comment #{comment_value}"
      end
    end

    def parse_misc
      next_word = grab_next_word

      next_word = grab_next_word if next_word == 'pub'

      case next_word
      when 'use'
        @tokens << { type: :use_statement, value: parse_use_statement }
      when 'impl'
        @tokens << { type: :contract_impl, name: parse_impl_name, state: nil, functions: [] }
      when 'struct'
        @tokens << { type: :struct, name: parse_struct }
      else
        # raise "Invalid token #{next_word}"
        @cur_pos += 1
      end
    end

    def parse_use_statement
      use_statement = ''

      until at_semicolon?
        use_statement += @content[@cur_pos]
        @cur_pos += 1
      end

      use_statement.strip
    end

    def parse_struct
      struct_name = ''

      while !at_open_brace? && !at_newline? && !at_end_of_file? && !at_semicolon?
        struct_name += @content[@cur_pos]
        @cur_pos += 1
      end

      struct_name.strip
    end

    def parse_impl_name
      impl_name = ''

      until at_open_brace?
        impl_name += @content[@cur_pos]
        @cur_pos += 1
      end

      impl_name.strip
    end

    def at_semicolon?
      @content[@cur_pos] == ';'
    end

    def at_open_brace?
      @content[@cur_pos] == '{'
    end

    def at_close_brace?
      @content[@cur_pos] == '}'
    end

    def at_end_of_file?
      @cur_pos == @content.length
    end

    def at_newline?
      @content[@cur_pos] == "\n"
    end

    def at_space?
      @content[@cur_pos] == ' '
    end

    def grab_next_word
      word = ''

      while !at_space? && !at_newline? && !at_end_of_file?
        word += @content[@cur_pos]
        @cur_pos += 1
      end

      word
    end
  end
end
