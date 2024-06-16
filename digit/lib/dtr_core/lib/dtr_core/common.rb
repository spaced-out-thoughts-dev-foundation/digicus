# frozen_string_literal: true

module DTRCore
  # Common methods used by the DTRCore module.
  module Common
    def split_strip_select(some_list)
      some_list&.split("\n")&.map(&:strip)&.select { |x| x.length.positive? }
    end

    def capture_section(pattern)
      captures = content.match(pattern)&.captures

      if content.scan(pattern).length > 1
        raise 'Multiple captures found for a section.'
      elsif captures
        captures&.first
      end
    end

    def clean_name(definition)
      definition.gsub(/[\*\n\[]/, '').strip
    end
  end
end
