# frozen_string_literal: true

module DTRCore
  # Common methods used by the DTRCore module.
  module Common
    def strip_and_remove_quotes(str)
      str.strip.gsub(/['"]/, '')
    end

    def split_strip_select(some_list)
      some_list&.split("\n")&.map { |x| x&.strip }&.select { |x| x.length.positive? }
    end
  end
end
