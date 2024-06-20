# frozen_string_literal: true

module DTRToRust
  module Common
    # TypeTranslator translates DTR types to Rust types
    module TypeTranslator
      def self.translate_type(type)
        # TODO: fix this, it is incorrect
        type
          .gsub('List<', 'Vec<')
          .gsub('Dictionary<', 'HashMap<')
          .gsub('BigInteger', 'i128')
          .gsub('Integer', 'i64')
          .gsub('String', 'Symbol')
          .gsub('Boolean', 'bool')
          .gsub('Float', 'f64')
      end
    end
  end
end
