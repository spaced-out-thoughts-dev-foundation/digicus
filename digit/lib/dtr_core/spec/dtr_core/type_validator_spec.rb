# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRCore::TypeValidator do
  context 'when input faulty' do
    it 'raises error when type name is missing' do
      type_validator = described_class.new(nil, '123')

      expect { type_validator.validate_then_coerce_initial_value! }.to raise_error('Missing Type Name.')
    end

    it 'raises error when initial value is missing' do
      type_validator = described_class.new('Integer', nil)

      expect { type_validator.validate_then_coerce_initial_value! }.to raise_error('Missing Initial Value.')
    end
  end

  context 'when type is Integer' do
    let(:type_validator) { described_class.new('Integer', '123') }

    it 'validates and coerces initial value' do
      expect(type_validator.validate_then_coerce_initial_value!).to eq(123)
    end

    it 'raises error when initial value is invalid' do
      type_validator = described_class.new('Integer', 'abc')

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for Integer: abc. Wrong type.')
    end

    it 'raises error when initial value is out of range' do
      type_validator = described_class.new('Integer',
                                           '99999999999999999999999999')

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for type Integer. Out of range.')
    end
  end

  context 'when type is BigInteger' do
    let(:type_validator) { described_class.new('BigInteger', '123') }

    it 'validates and coerces initial value' do
      expect(type_validator.validate_then_coerce_initial_value!).to eq(123)
    end

    it 'raises error when initial value is invalid' do
      type_validator = described_class.new('BigInteger', 'abc')

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for BigInteger: abc. Wrong type.')
    end

    it 'raises error when initial value is out of range' do
      type_validator = described_class.new('BigInteger',
                                           (9**100_000).to_s)

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for type BigInteger. Out of range.')
    end
  end

  context 'when type is Float' do
    let(:type_validator) { described_class.new('Float', '123.45') }

    it 'validates and coerces initial value' do
      expect(type_validator.validate_then_coerce_initial_value!).to eq(123.45)
    end

    it 'raises error when initial value is invalid' do
      type_validator = described_class.new('Float', 'abc')

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for Float: abc. Wrong type.')
    end

    it 'raises error when initial value is out of range' do
      type_validator = described_class.new('Float',
                                           "#{9**100_000}.#{9**1000}")

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for type Float. Out of range.')
    end
  end

  context 'when type is String' do
    let(:type_validator) { described_class.new('String', '"abc"') }

    it 'validates and coerces initial value' do
      expect(type_validator.validate_then_coerce_initial_value!).to eq '"abc"'
    end

    it 'raises error when initial value is invalid' do
      type_validator = described_class.new('String', 'abc')

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for String: abc. Wrong type.')
    end
  end

  context 'when type is Address' do
    let(:type_validator) { described_class.new('Address', 'GCDXNQDQXROCXFPDJFE6CATCT2IF5VR3YJ2FQKZVM60BUC5MEGFSPBUJ') }

    it 'validates and coerces initial value' do
      expect(type_validator.validate_then_coerce_initial_value!)
        .to eq 'GCDXNQDQXROCXFPDJFE6CATCT2IF5VR3YJ2FQKZVM60BUC5MEGFSPBUJ'
    end

    it 'raises error when initial value is invalid' do
      type_validator = described_class.new('Address', 'GABCD1234XYZ')

      expect do
        type_validator.validate_then_coerce_initial_value!
      end.to raise_error('Invalid initial value for Address: GABCD1234XYZ. Wrong type.')
    end
  end
end
