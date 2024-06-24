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
        "#{derives}pub struct #{@user_defined_type.name.gsub('_STRUCT', '')} {\n#{generate_struct_attributes}\n}\n\n"
      end

      def generate_struct_attributes
        @user_defined_type.attributes.map do |x|
          "    pub #{x[:name]}: #{Common::TypeTranslator.translate_type(x[:type])},"
        end.join("\n")
      end

      def generate_enum
        "#{derives}pub enum #{@user_defined_type.name.gsub('_ENUM', '')} {\n#{generate_enum_attributes}\n}\n\n"
      end

      def generate_enum_attributes
        @user_defined_type.attributes.map do |x|
          if x[:value]
            "    #{x[:name]} = #{x[:value]},"
          elsif x[:type] && x[:type].start_with?('(') && x[:type].end_with?(')')
            inner_types = x[:type].gsub('(', '').gsub(')', '').split(',').map do |x|
              Common::TypeTranslator.translate_type(x)
            end
            if inner_types.size == 0 || x[:type] == '()'
              "    #{x[:name]},"
            else
              "    #{x[:name]}(#{inner_types.join(', ')}),"
            end
          elsif x[:type] && x[:type].match(/\d+/)
            "    #{x[:name]} = #{x[:type]},"
          elsif x[:type] && x[:type] != '()'
            "    #{x[:name]}(#{Common::TypeTranslator.translate_type(x[:type])}),"
          else
            "    #{x[:name]},"
          end
        end.join("\n")
      end

      def derives
        base = if error? && enum?
                 "#[contracterror]\n#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]\n"
               else
                 "#[contracttype]\n#[derive(Clone, Debug, Eq, PartialEq)]\n"
               end

        base += "#[repr(u32)]\n" if numbered_enum?

        base
      end

      # TODO: fix this terrible hack
      def error?
        @user_defined_type.name.start_with? 'Error'
      end

      def struct?
        @user_defined_type.name.end_with? '_STRUCT'
      end

      def enum?
        @user_defined_type.name.end_with? '_ENUM'
      end

      def numbered_enum?
        enum? && @user_defined_type.attributes.all? { |x| x[:value] }
      end
    end
  end
end
