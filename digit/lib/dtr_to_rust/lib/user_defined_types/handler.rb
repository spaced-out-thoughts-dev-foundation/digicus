# frozen_string_literal: true

module DTRToRust
  module UserDefinedTypes
    class Handler
      def initialize(user_defined_type)
        @user_defined_type = user_defined_type
      end

      def self.generate(user_defined_type)
        new(user_defined_type).generate
      end

      def generate
        if struct?
          generate_struct
        elsif enum?
          generate_enum
        end
      end

      def generate_struct
        "#{derives}pub struct #{@user_defined_type.name.gsub('_STRUCT', '')} {#{@user_defined_type.attributes.map do |x|
                                                                                  "#{x[:name]}: #{Common::TypeTranslator.translate_type(x[:type])}"
                                                                                end.join(', ')}}\n\n"
      end

      def generate_enum
        "#{derives}pub enum #{@user_defined_type.name.gsub('_ENUM', '')} {\n#{generate_enum_attributes}\n}\n\n"
      end

      def generate_enum_attributes
        @user_defined_type.attributes.map do |x|
          "    #{x[:name]},"
        end.join('\n')
      end

      def derives
        "#[contracttype]\n#[derive(Clone, Debug, Eq, PartialEq)]\n"
      end

      def struct?
        @user_defined_type.name.end_with? '_STRUCT'
      end

      def enum?
        @user_defined_type.name.end_with? '_ENUM'
      end
    end
  end
end
