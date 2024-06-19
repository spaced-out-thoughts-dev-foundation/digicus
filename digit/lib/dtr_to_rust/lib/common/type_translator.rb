# frozen_string_literal: true

module DTRToRust
  module Common
    module TypeTranslator
      def self.translate_type(type)
        # TODO: fix this, it is incorrect
        type
          .gsub('List<', 'Vec<')
          .gsub('Dictionary<', 'HashMap<')
          .gsub('Integer', 'i64')
          .gsub('BigInteger', 'i256')
          .gsub('String', 'Symbol')
          .gsub('Boolean', 'bool')
          .gsub('Float', 'f64')
      end
    end
  end
end
