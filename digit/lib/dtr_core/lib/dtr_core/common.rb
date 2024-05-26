# frozen_string_literal: true

module DTRCore
  # Common methods used by the DTRCore module.
  module Common
    def strip_and_remove_quotes(str)
      str.strip.gsub(/['"]/, '')
    end

    def split_strip_select(some_list)
      some_list&.split("\n")&.map(&:strip)&.select { |x| x.length.positive? }
    end

    def first_match_for_content(pattern)
      content.match(pattern)&.captures&.first
    end

    def clean_name(definition)
      definition.gsub(/[\*\n\[]/, '').strip
    end
  end
end
